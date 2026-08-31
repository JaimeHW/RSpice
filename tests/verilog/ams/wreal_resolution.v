// What a real net with more than one driver is worth.
//
// Verilog-AMS LRM 2.4 section 6.5.3 permits "a maximum of one driver of a
// real-valued net", and section 3.7 gives no resolution function to combine
// two with — the standard has none. A plain `wreal` with two drivers is
// therefore refused by the front end and cannot appear in a corpus case.
//
// The four resolved spellings below are the real-number-modelling extension
// RSpice implements beside the standard's net type, opted into by writing the
// resolution on the declaration. Each net here has exactly the same two
// drivers, so the four columns of the trace differ only by the fold their
// keyword named.
//
// `floating` has no driver at all, which is the case section 3.7 does answer:
// "If no driver is connected to a wreal net, its value shall be zero (0.0).
// Unlike other digital nets which have an initial value of `z`, wreal nets
// shall have an initial value of zero."
module wreal_resolution(hi, lo, summed, averaged, least, greatest, floating);
  input hi;
  input lo;
  output wrealsum summed;
  output wrealavg averaged;
  output wrealmin least;
  output wrealmax greatest;
  output wreal floating;

  wire hi, lo;

  assign summed    = hi ? 3.0 : 1.0;
  assign summed    = lo ? -1.0 : -4.0;
  assign averaged  = hi ? 3.0 : 1.0;
  assign averaged  = lo ? -1.0 : -4.0;
  assign least     = hi ? 3.0 : 1.0;
  assign least     = lo ? -1.0 : -4.0;
  assign greatest  = hi ? 3.0 : 1.0;
  assign greatest  = lo ? -1.0 : -4.0;
endmodule
