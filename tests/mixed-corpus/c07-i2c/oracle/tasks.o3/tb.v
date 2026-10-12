// C7 O3 (Icarus standalone): the variant's Verilog on a tri1 bus. The bus
// nets carry the names of the RSpice deck's bus observers (sda_d, scl_d), so
// the protocol bits compare by name. Nothing is replayed: the modules drive
// the whole protocol themselves, with the analog RC cut out.
// rspice-o3-sources: ../../net/i2c_ctrl_tasks.v ../../net/i2c_dev.v
// rspice-o3-timescale: 1ps
`timescale 1ns/1ps
module tb;
  tri1 scl_d, sda_d;
  i2c_ctrl ctrl(.scl(scl_d), .sda(sda_d));
  i2c_dev #(.ADDR(42)) dev2a(.scl(scl_d), .sda(sda_d));
  i2c_dev #(.ADDR(81)) dev51(.scl(scl_d), .sda(sda_d));
  // RSPICE-O3-STIMULUS
  initial begin
    $dumpfile("o3.vcd");
    $dumpvars(1, scl_d, sda_d);
    #540000 $finish;
  end
endmodule
