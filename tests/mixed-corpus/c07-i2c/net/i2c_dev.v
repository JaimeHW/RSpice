// C7: I2C target (RSpice-authored). One module serves both devices through
// numeric parameter overrides on its X-card (ADDR = 7-bit address).
// SDA is open-drain: the device pulls it low or leaves it at z. It ACKs its
// address and a written byte, stores the byte, and returns it on a read
// until the controller NACKs. With CONTEND = 1 it also drives SDA push-pull
// high over [CT0, CT0 + CTW] (the contention deck).
`timescale 1ns/1ps
module i2c_dev(scl, sda);
  input scl;
  inout sda;
  parameter ADDR = 42;
  parameter CONTEND = 0;
  parameter CT0 = 300000;
  parameter CTW = 10000;
  reg sda_oe, force_hi;
  assign sda = force_hi ? 1'b1 : (sda_oe ? 1'b0 : 1'bz);
  reg [7:0] shift, mem, out;
  reg [3:0] bitcnt;
  reg [1:0] phase;   // 0 idle, 1 address, 2 write data, 3 read data
  reg matched, nack;
  initial begin
    sda_oe = 1'b0;
    force_hi = 1'b0;
    shift = 8'h00;
    mem = 8'h00;
    out = 8'h00;
    bitcnt = 4'd0;
    phase = 2'd0;
    matched = 1'b0;
    nack = 1'b0;
    if (CONTEND) begin
      #(CT0) force_hi = 1'b1;
      #(CTW) force_hi = 1'b0;
    end
  end
  // START: SDA falls while SCL is high.
  always @(negedge sda)
    if (scl === 1'b1) begin
      phase = 2'd1;
      bitcnt = 4'd0;
      matched = 1'b0;
      sda_oe = 1'b0;
    end
  // STOP: SDA rises while SCL is high.
  always @(posedge sda)
    if (scl === 1'b1) begin
      phase = 2'd0;
      sda_oe = 1'b0;
    end
  // Bits are sampled on the SCL rise.
  always @(posedge scl)
    if (phase != 2'd0) begin
      bitcnt = bitcnt + 4'd1;
      if (bitcnt <= 4'd8 && phase != 2'd3)
        shift = {shift[6:0], sda};
      if (bitcnt == 4'd9 && phase == 2'd3)
        nack = sda;
    end
  // SDA changes only while SCL is low.
  always @(negedge scl)
    if (phase != 2'd0) begin
      if (bitcnt == 4'd8) begin
        if (phase == 2'd1) begin
          matched = (shift[7:1] == ADDR);
          sda_oe = matched;
        end else if (phase == 2'd2) begin
          if (matched)
            mem = shift;
          sda_oe = matched;
        end else
          sda_oe = 1'b0;
      end else if (bitcnt == 4'd9) begin
        bitcnt = 4'd0;
        if (phase == 2'd1 && !matched)
          phase = 2'd0;
        else if (phase == 2'd1 && shift[0]) begin
          phase = 2'd3;
          out = mem;
          sda_oe = ~out[7];
        end else if (phase == 2'd1) begin
          phase = 2'd2;
          sda_oe = 1'b0;
        end else if (phase == 2'd2)
          sda_oe = 1'b0;
        else if (nack) begin
          phase = 2'd0;
          sda_oe = 1'b0;
        end else begin
          out = mem;
          sda_oe = ~out[7];
        end
      end else if (phase == 2'd3) begin
        out = out << 1;
        sda_oe = ~out[7];
      end
    end
endmodule
