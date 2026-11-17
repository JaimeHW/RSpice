//! What a Project page derives from the project, kept between frames.
//!
//! A page that walks the design, a catalogue or the run history to show a
//! count pays for the walk on every repaint, and nothing it paints shows the
//! cost. So each such value is kept in the frame context beside the identity
//! of what it was derived from — a content stamp, a catalogue revision, a
//! digest — and derived again only when that identity moves: a frame in which
//! nothing changed derives nothing.

/// The value `derive` gives for `key`, kept in `ctx` under `name` and derived
/// again only when `key` differs from the one it was last derived for.
pub(crate) fn derived<K, V>(
    ctx: &egui::Context,
    name: &'static str,
    key: K,
    derive: impl FnOnce() -> V,
) -> V
where
    K: PartialEq + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    let id = egui::Id::new(("workbench.project.derived", name));
    if let Some((kept, value)) = ctx.data(|data| data.get_temp::<(K, V)>(id))
        && kept == key
    {
        return value;
    }
    let value = derive();
    ctx.data_mut(|data| data.insert_temp(id, (key, value.clone())));
    value
}
