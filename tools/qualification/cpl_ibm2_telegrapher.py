"""Print an independent IBM2 tail reference from the telegrapher equations.

Requires NumPy. Does not import RSpice/ngspice code or modify checked-in data.
Run with --refine to report all Fourier-grid/time-window refinements.
Equal diagonal/off-diagonal L and C have exact common/differential modes.
For each mode Y11=Yc*coth(gamma*l), Y12=-Yc*csch(gamma*l).
The pulse Laplace transform is exact; exponential damping suppresses images.
"""
import argparse
import numpy as np


def reference(window, power):
    count = 2**power
    dt = window / count
    alpha = 4e8
    s = alpha + 2j * np.pi * np.fft.rfftfreq(count, dt)
    z = 0.5 + s[:, None] * np.array([278.95e-9, 215.65e-9])
    y = s[:, None] * np.array([28.95e-12, 33.85e-12])
    gamma = np.sqrt(z * y)
    yc = np.sqrt(y / z)
    diagonal = yc / np.tanh(gamma * 0.3048)
    cross = -yc / np.sinh(gamma * 0.3048)
    matrix = np.zeros((len(s), 4, 4), dtype=complex)
    for row, col, modes in [(0, 0, diagonal), (2, 2, diagonal), (0, 2, cross), (2, 0, cross)]:
        d = (modes[:, 0] + modes[:, 1]) / 2
        o = (modes[:, 0] - modes[:, 1]) / 2
        matrix[:, row, col] = d
        matrix[:, row + 1, col + 1] = d
        matrix[:, row, col + 1] = o
        matrix[:, row + 1, col] = o
    for port, resistance in enumerate([50, 10, 100, 100]):
        matrix[:, port, port] += 1 / resistance
    source = (1 - np.exp(-s * 1.5e-9) - np.exp(-s * 6e-9) + np.exp(-s * 7.5e-9)) / (1.5e-9 * s * s)
    rhs = np.zeros((len(s), 4, 1), dtype=complex)
    rhs[:, 0, 0] = source / 50
    transformed = np.linalg.solve(matrix, rhs)[:, :, 0]
    # Only the 20 ns authored interval is needed; avoid multiplying distant
    # periodic images by a large inverse damping factor.
    time = np.arange(int(round(20e-9 / dt)) + 1) * dt
    voltage = np.fft.irfft(transformed, n=count, axis=0)[:len(time)] / dt * np.exp(alpha * time[:, None])
    if not np.isfinite(voltage).all():
        raise ValueError("nonfinite inversion")
    return np.array([[at, *[np.interp(at, time, voltage[:, port]) for port in range(4)]]
                     for at in [1.005e-8, 1.015e-8, 1.025e-8]])


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--refine", action="store_true")
    args = parser.parse_args()
    cases = [(80e-9, 16), (80e-9, 17), (80e-9, 18), (160e-9, 19)] if args.refine else [(160e-9, 19)]
    previous = None
    for window, power in cases:
        values = reference(window, power)
        print(f"# window={window:.17e}, N={2**power}")
        if previous is not None:
            error = np.max(np.abs(values[:, 1:] - previous[:, 1:]))
            print(f"# maximum change from previous refinement: {error:.6e} V")
            # The coarsest grid is diagnostic. Require the final Fourier
            # refinement and subsequent window doubling to meet 30 nV.
            if power >= 18 and error > 3e-8:
                raise ValueError("final reference refinement exceeded 30 nV")
        for row in values:
            print(" ".join(f"{value:.17e}" for value in row))
        previous = values
