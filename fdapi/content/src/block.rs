use crate::{item::Item, fluid::Fluid};

pub trait Block {}

pub trait ItemsConsumer<'a> : Block + TryConsume<&'a dyn Item> {}

pub trait LiquidConsumer<'a> : Block + TryConsume<&'a dyn Fluid> {}

pub trait PowerConsumer : Block {}

pub trait TryConsume<I> {
  fn consume(val: I);

  fn is_can_consume(val: I) -> bool;
}