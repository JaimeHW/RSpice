// C7: I2C controller with its script written as tasks (RSpice-authored).
// The same bus behaviour as i2c_ctrl.v's MODE = 0 script; tasks are refused
// by the digital front end until P2 lane L3.
`timescale 1ns/1ps
module i2c_ctrl(scl, sda);
  inout scl, sda;
  parameter Q = 2500;
  reg scl_oe, sda_oe;
  assign scl = scl_oe ? 1'b0 : 1'bz;
  assign sda = sda_oe ? 1'b0 : 1'bz;
  reg [7:0] rx;
  reg ack;
  integer i;

  task start_condition;
    begin
      sda_oe = 1'b1;
      #(2*Q) scl_oe = 1'b1;
    end
  endtask

  task stop_condition;
    begin
      #(Q) sda_oe = 1'b1;
      #(Q) scl_oe = 1'b0;
      #(2*Q) sda_oe = 1'b0;
    end
  endtask

  task send_byte(input [7:0] value);
    reg [7:0] tx;
    begin
      tx = value;
      for (i = 0; i < 8; i = i + 1) begin
        #(Q) sda_oe = ~tx[7];
        tx = tx << 1;
        #(Q) scl_oe = 1'b0;
        #(2*Q) scl_oe = 1'b1;
      end
      #(Q) sda_oe = 1'b0;
      #(Q) scl_oe = 1'b0;
      #(Q) ack = sda;
      #(Q) scl_oe = 1'b1;
    end
  endtask

  task receive_byte_nack;
    begin
      for (i = 0; i < 8; i = i + 1) begin
        #(Q) sda_oe = 1'b0;
        #(Q) scl_oe = 1'b0;
        #(Q) rx = {rx[6:0], sda};
        #(Q) scl_oe = 1'b1;
      end
      #(Q) sda_oe = 1'b0;
      #(Q) scl_oe = 1'b0;
      #(2*Q) scl_oe = 1'b1;
    end
  endtask

  initial begin
    scl_oe = 1'b0;
    sda_oe = 1'b0;
    rx = 8'h00;
    ack = 1'b1;
    #10000;
    start_condition;
    send_byte(8'h54);
    send_byte(8'hC5);
    stop_condition;
    #10000;
    start_condition;
    send_byte(8'h55);
    receive_byte_nack;
    stop_condition;
    #10000;
    start_condition;
    send_byte(8'h66);
    stop_condition;
  end
endmodule
