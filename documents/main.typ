#import "@preview/catppuccin:1.1.0": catppuccin, flavors
#import "@preview/lilaq:0.6.0" as lq
#import "@preview/unify:0.8.1" as u
#import "@preview/chemformula:0.1.3": ch
#let flavor = flavors.macchiato
#show: catppuccin.with(flavor)
#let date = datetime.today()
#set text(font: "New Computer Modern")
#let palette = flavor.colors
#let cat-cycle = (
  palette.blue.rgb,
  palette.lavender.rgb,
  palette.mauve.rgb,
  palette.pink.rgb,
  palette.flamingo.rgb,
  palette.rosewater.rgb,
)
