"""Regenerate the independent FFT distribution oracle with mpmath==1.3.0.

Uses 100-digit differentiation of the original cosine/Bessel definitions,
not the production sine-polynomial jets or hypergeometric derivative recurrence.
Run this file with Python; the adjacent JSON is the only input to Rust tests.
"""
from pathlib import Path
import json
import mpmath as mp

mp.mp.dps = 100
q = mp.mpf

def window(name, x, alpha):
    c = lambda n: mp.cos(2*mp.pi*n*x)
    if name == "rect": return q(1)
    if name == "bartlett": return 1-abs(2*x-1)
    if name == "bartletthann": return q(".62")-q(".48")*abs(x-q(".5"))+q(".38")*mp.cos(2*mp.pi*(x-q(".5")))
    if name == "hamming": return q(".54")-q(".46")*c(1)
    if name in ("hann", "cosine2"): return (1-c(1))/2
    if name == "black": return q(".42323")-q(".49755")*c(1)+q(".07922")*c(2)
    if name == "blackman": return q(".42")-q(".5")*c(1)+q(".08")*c(2)
    if name == "harris": return q(".35875")-q(".48829")*c(1)+q(".14128")*c(2)-q(".01168")*c(3)
    if name == "nuttall": return q(".3635819")-q(".4891775")*c(1)+q(".1365995")*c(2)-q(".0106411")*c(3)
    if name == "halfcyclesine": return mp.sin(mp.pi*x)
    if name == "halfcyclesine3": return mp.sin(mp.pi*x)**3
    if name == "halfcyclesine6": return mp.sin(mp.pi*x)**6
    if name == "cosine4": return q(".375")-q(".5")*c(1)+q(".125")*c(2)
    if name == "gaussian": return mp.exp(-q(".5")*(alpha*(2*x-1))**2)
    if name == "kaiser": return mp.besseli(0,alpha*mp.sqrt(1-(2*x-1)**2))/mp.besseli(0,alpha)
    raise AssertionError(name)

names = ("rect bartlett bartletthann hamming hann black blackman harris nuttall "
         "halfcyclesine halfcyclesine3 halfcyclesine6 cosine2 cosine4 gaussian kaiser").split()
cases = []
duration = q("1.25")
coefficient = q("-0.000001")
for name in names:
    for alpha in ([1,3,20] if name in ("gaussian", "kaiser") else [3]):
        for mode in [0,1]:
            denominator = 8 if mode else 7
            gain = sum(window(name,q(index)/denominator,alpha) for index in range(8))/8
            for fraction in [q(5)/16,q(11)/16]:
                for order in [1,2,3,5,8]:
                    bins = []
                    for k in range(5):
                        basis = lambda u: window(name,u*8/denominator,alpha)*mp.exp(-2j*mp.pi*k*u)
                        value = coefficient*(-1)**order*mp.diff(basis,fraction,order)/(duration**(order+1)*gain)
                        if k not in (0,4): value *= 2
                        bins.append([float(mp.re(value)),float(mp.im(value))])
                    cases.append(dict(window=name,alpha=alpha,mode=mode,order=order,
                                      fraction=float(fraction),gain=float(gain),bins=bins))
out = dict(generator="mpmath 1.3.0, 100 decimal digits, original window definitions",
           points=8,start=0.5,duration=float(duration),coefficient=float(coefficient),cases=cases)
path = Path(__file__).with_name("fft-window-derivative-oracle.json")
header = {key: value for key, value in out.items() if key != "cases"}
document = json.dumps(header, indent=2)[:-2] + ',\n  "cases": [\n'
document += ',\n'.join('    ' + json.dumps(case, separators=(',', ':')) for case in cases)
path.write_text(document + '\n  ]\n}\n', encoding='utf-8')
print(f"{len(cases)} spectra, {5*len(cases)} complex coefficients: {path}")
