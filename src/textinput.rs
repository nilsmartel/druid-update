use druid::{Data, EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx, UpdateCtx, Widget, WinCtx};
use std::sync::Arc;

#[derive(Clone)]
pub struct Updater<W, T, F>
where
    T: Data,
    W: Widget<T>,
    F: Fn(&T),
{
    widget: W,
    onupdate: F,
    marker: std::marker::PhantomData<T>,
}

impl<W, T, F> std::ops::Deref for Updater<W, T, F>
where
    T: Data,
    W: Widget<T>,
    F: Fn(&T),
{
    type Target = W;
    fn deref(&self) -> &Self::Target {
        &self.widget
    }
}

impl<W, T, F> std::ops::DerefMut for Updater<W, T, F>
where
    T: Data,
    W: Widget<T>,
    F: Fn(&T),
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.widget
    }
}

pub trait OnUpdate<T>
where
    T: Data,
    Self: Sized + Widget<T>,
{
    fn onupdate<F: Fn(&T)>(self, f: F) -> Updater<Self, T, F>;
}

impl<W, T> OnUpdate<T> for W
where
    W: Widget<T>,
    T: Data,
    Self: Sized,
{
    fn onupdate<F: Fn(&T)>(self, f: F) -> Updater<Self, T, F> {
        Updater {
            widget: self,
            onupdate: f,
            marker: std::marker::PhantomData::default(),
        }
    }
}

impl<W, T, F> Widget<T> for Updater<W, T, F>
where
    W: Widget<T>,
    T: Data,
    F: Fn(&T),
{
    fn event(&mut self, ctx: &mut EventCtx, event: &druid::Event, data: &mut T, env: &druid::Env) {
        println!("Event");
        self.widget.event(ctx, event, data, env);
    }
    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &T,
        env: &druid::Env,
    ) {
        println!("Lifecycle");
        self.widget.lifecycle(ctx, event, data, env);
    }
    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &druid::Env) {
        println!("Update");
        let f = &self.onupdate;
        f(data);
        self.widget.update(ctx, old_data, data, env);
    }
    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &T,
        env: &druid::Env,
    ) -> druid::Size {
        self.widget.layout(ctx, bc, data, env)
    }
    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &T, env: &druid::Env) {
        self.widget.paint(paint_ctx, data, env)
    }
}
