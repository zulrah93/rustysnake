# Rusty Snake

## Project Description

The classic snake game implementation in Rust using the Piston game engine. With a multiplayer feature in the near future.

## How to build

In project folder run:

    cargo run --release

## What's Complete

- Single player

## Controls

Up and down to select menu and press Space to select option. Arrow keys to control snake when playing. Space to toggle faster snake speed

## Rules

Same rules as the classic game but scoring is based off how fast you can get the next food. Each eaten food is capped at 100 but the score lowers as you are slower to get the food. You can increase the snake's speed by toggling the Space key but by doing so you risk losing control. You also lose by moving out of bounds. 