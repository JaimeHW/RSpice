# DC mismatch statistical moments

DC mismatch uses the design's Spectre statistics for process variation and
independent instance mismatch. Gaussian, uniform, and lognormal populations may
carry bounds and correlations. Bounds condition the joint population, so a
bound on one parameter can also change a correlated, unbounded parameter's
spread. Correlations describe the population before conditioning, as in the
Monte Carlo sampler.

The Studio exposes **Moment relative tolerance** and **Moment point budget**.
Blank fields use the engine defaults: `0.001` and `262144`. Tolerance must be
finite and in `(0, 0.1]`; the integer point budget must be at least `1024`.
Exported cards carry nondefault controls as `MOMENT_RELTOL` and
`MOMENT_MAX_POINTS`. The point budget applies separately to each enabled
statistical scope and is capped by the engine's analysis-point resource limit.
These are statistical integration points, not circuit simulations.

Saved drafts preserve the text controls; typed plans, manual-deck imports,
execution identities, Python card descriptions, and worker requests preserve
the resolved values. Older saved plans use the defaults. Browser request
protocol 34 rejects older workers that could discard the controls. The response
protocol is 29, which also retains scalar measurement units.

Unbounded moments are analytic, including the physical lognormal standard
deviation. Bounded groups use the sampler's Gaussian copula with Genz conditional
interval integration and eight shifted Halton rules. Replicate disagreement and
refinement changes estimate convergence; the tolerance is not a rigorous error
bound. Infeasible bounds, resource exhaustion, cancellation, and non-convergence
are reported rather than silently substituting unbounded statistics. The output
spread still uses first-order circuit sensitivities at the nominal operating
point; conditional moments do not turn DCMATCH into nonlinear Monte Carlo.

Focused verification covers analytic conditioned Gaussian covariance,
independent density integrals for mixed distributions, singular correlations,
tail probabilities, process and instance circuit variances, and a Studio request
whose selected budget changes a convergence refusal into a successful run.

Numerical method reference: [Genz, numerical computation of multivariate normal
probabilities](https://people.cs.kuleuven.be/~dirk.nuyens/mcqmc2014_proceedings_preprints/223.pdf).
