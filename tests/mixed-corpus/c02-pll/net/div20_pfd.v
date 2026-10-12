// C2 mx: divide-by-20 and phase-frequency detector in Verilog.
// clkin is the XSPICE divide-by-2 output (VCO/2), so divq runs at VCO/40. As in the
// ngspice f-p-det, the feedback edge sets `up` (pump charges cont, VCO slows)
// and the reference edge sets `dn`; both clear 100 ps after both are set.
`timescale 1ps/1ps
module div20_pfd(clkin, ref, up_n, dn, divo);
  input clkin, ref; output up_n, dn, divo;
  reg up, dni, rst, divq;
  reg [3:0] count;
  initial begin up = 1'b0; dni = 1'b0; rst = 1'b0; divq = 1'b0; count = 4'd0; end
  always @(posedge clkin) begin
    if (count == 4'd9) begin
      count <= 4'd0;
      divq <= ~divq;
    end else
      count <= count + 4'd1;
  end
  always @(posedge divq or posedge rst) if (rst) up <= 1'b0; else up <= 1'b1;
  always @(posedge ref or posedge rst) if (rst) dni <= 1'b0; else dni <= 1'b1;
  always @(up or dni) rst <= #100 (up & dni);
  assign up_n = ~up;
  assign dn = dni;
  assign divo = divq;
endmodule
