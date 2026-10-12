// C7: I2C controller, task-free (RSpice-authored). SCL is 100 kHz (Q is a
// quarter period); SCL and SDA are open-drain. Script:
//   1. START, 0x2A write, 0xC5, STOP
//   2. START, 0x2A read, one byte, NACK, STOP
//   3. START, 0x33 write (no such device: NACK), STOP
// SDA changes a quarter period after SCL falls; the controller samples ACK
// and read data in the middle of SCL high. With MODE = 1 it instead holds
// SDA low over [HOLD0, HOLD0 + HOLDW] (the contention deck).
// The same behaviour written with tasks is i2c_ctrl_tasks.v.
`timescale 1ns/1ps
module i2c_ctrl(scl, sda);
  inout scl, sda;
  parameter MODE = 0;
  parameter HOLD0 = 295000;
  parameter HOLDW = 20000;
  parameter Q = 2500;
  reg scl_oe, sda_oe;
  assign scl = scl_oe ? 1'b0 : 1'bz;
  assign sda = sda_oe ? 1'b0 : 1'bz;
  reg [7:0] tx, rx;
  reg ack;
  integer i;
  initial begin
    scl_oe = 1'b0;
    sda_oe = 1'b0;
    tx = 8'h00;
    rx = 8'h00;
    ack = 1'b1;
    if (MODE == 1) begin
      #(HOLD0) sda_oe = 1'b1;
      #(HOLDW) sda_oe = 1'b0;
    end else begin
      #10000;
      // 1: write 0xC5 to 0x2A.
      sda_oe = 1'b1;
      #(2*Q) scl_oe = 1'b1;
      tx = 8'h54;
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
      tx = 8'hC5;
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
      #(Q) sda_oe = 1'b1;
      #(Q) scl_oe = 1'b0;
      #(2*Q) sda_oe = 1'b0;
      #10000;
      // 2: read one byte from 0x2A, then NACK.
      sda_oe = 1'b1;
      #(2*Q) scl_oe = 1'b1;
      tx = 8'h55;
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
      for (i = 0; i < 8; i = i + 1) begin
        #(Q) sda_oe = 1'b0;
        #(Q) scl_oe = 1'b0;
        #(Q) rx = {rx[6:0], sda};
        #(Q) scl_oe = 1'b1;
      end
      #(Q) sda_oe = 1'b0;
      #(Q) scl_oe = 1'b0;
      #(2*Q) scl_oe = 1'b1;
      #(Q) sda_oe = 1'b1;
      #(Q) scl_oe = 1'b0;
      #(2*Q) sda_oe = 1'b0;
      #10000;
      // 3: address 0x33, which no device answers.
      sda_oe = 1'b1;
      #(2*Q) scl_oe = 1'b1;
      tx = 8'h66;
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
      #(Q) sda_oe = 1'b1;
      #(Q) scl_oe = 1'b0;
      #(2*Q) sda_oe = 1'b0;
    end
  end
endmodule
