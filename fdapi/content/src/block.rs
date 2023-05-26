use std::{fmt::Formatter, any::TypeId};

use crate::{item::Item, fluid::Fluid};

pub trait Block {}

pub struct BlockPlan<'a> {
  block: &'a dyn Block
}

impl<'a> std::fmt::Debug for BlockPlan<'a> {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
    formatter.debug_struct("BlockPlan")
      .finish()
  }
}

pub trait ItemsConsumer<'a> : Block + TryConsume<&'a dyn Item> {}

pub trait LiquidConsumer<'a> : Block + TryConsume<&'a dyn Fluid> {}

pub trait PowerConsumer : Block {}

pub trait TryConsume<I> {
  fn consume(val: I);

  fn is_can_consume(val: I) -> bool;
}