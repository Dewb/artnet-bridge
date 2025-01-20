-- Wireshark protocol dissector for KiNET UDP packets
-- to install:
--   $ cp examples/kinet_dissector.lua ~/.local/lib/wireshark/plugins

kinet_protocol = Proto("KiNET", "KiNET Protocol")

magic = ProtoField.uint32("kinet.magic", "magic", base.HEX)
version = ProtoField.uint16("kinet.version", "version", base.DEC)
command = ProtoField.uint16("kinet.command", "command", base.HEX, {
    [0x101] = "DmxOut",
    [0x108] = "PortOut"
})
sequence = ProtoField.uint32("kinet.sequence", "sequence", base.DEC)
port = ProtoField.uint8("kinet.port", "port", base.DEC)
padding = ProtoField.uint8("kinet.padding", "padding", base.HEX)
flags = ProtoField.uint16("kinet.flags", "flags", base.HEX)

kinet_protocol.fields = { magic, version, command, sequence, port, padding, flags }

kinet_protocol_portout = Proto("KiNET_PortOut", "KiNET Protocol PortOut Command")

portout_port = ProtoField.uint8("kinet.portout.port", "port", base.DEC)
portout_padding = ProtoField.uint8("kinet.portout.padding", "padding", base.DEC)
portout_flags = ProtoField.uint16("kinet.portout.flags", "flags", base.HEX)
portout_length = ProtoField.uint16("kinet.portout.length", "length", base.HEX)
portout_startcode = ProtoField.uint16("kinet.portout.startcode", "startcode", base.HEX)

kinet_protocol_portout.fields = { portout_port, portout_padding, portout_flags, portout_length, portout_startcode }

function kinet_protocol.dissector(buffer, pinfo, tree)
  length = buffer:len()
  if length == 0 then return end

  pinfo.cols.protocol = kinet_protocol.name

  local subtree = tree:add(kinet_protocol, buffer(), "KiNET Protocol Data")
  
  subtree:add_le(magic, buffer(0,4))
  subtree:add_le(version, buffer(4,2))
  subtree:add_le(command, buffer(6,2))
  subtree:add_le(sequence, buffer(8,4))
  subtree:add_le(port, buffer(12,1))
  subtree:add_le(padding, buffer(13,1))
  subtree:add_le(flags, buffer(14,2))

  if buffer(6,2):uint() == 0x801 then
    local commandtree = subtree:add(kinet_protocol_portout, buffer(), "KiNET PortOut Data")

    commandtree:add_le(portout_port, buffer(16,1))
    commandtree:add_le(portout_padding, buffer(17,1))
    commandtree:add_le(portout_flags, buffer(18,2))
    commandtree:add_le(portout_length, buffer(20,2))
    commandtree:add_le(portout_startcode, buffer(22,2))
  end
end

local udp_port = DissectorTable.get("udp.port")
udp_port:add(6038, kinet_protocol)