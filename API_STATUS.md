# GPUI API Status

This document tracks the status of standard GPUI APIs exposed to Bun via FFI.

## Implemented

### Application & Window
- [x] `App.run(callback)` - Start the application
- [x] `App.openWindow(appPtr, builder)` - Open a new window

### Element: Div / Layout
- [x] `flex()` - Display flex
- [x] `flexCol()` - Flex direction column
- [x] `sizeFull()` - Width and height 100%
- [x] `w(px)` - Fixed width in pixels
- [x] `h(px)` - Fixed height in pixels
- [x] `size(px)` - Fixed size (w+h) in pixels
- [x] `p(px)` - Padding (all sides)
- [x] `m(px)` - Margin (all sides)
- [x] `gap(px)` - Flex gap
- [x] `justifyCenter()`
- [x] `justifyBetween()`
- [x] `itemsCenter()`
- [x] `itemsStart()`
- [x] `itemsEnd()`

### Element: Styling
- [x] `bg(color)` - Background color (hex)
- [x] `textColor(color)` - Text color (hex)

### Structure
- [x] `child(string | Div)` - Add text or element child

## Pending

### Events
- [ ] `onClick(callback)`
- [ ] `onHover(callback)`
- [ ] `onScroll(callback)`
- [ ] `onKeyDown(callback)`

### Layout (Comprehensive)
- [ ] `relative()`, `absolute()`
- [ ] `inset(px)`, `top(px)`, `left(px)`, etc.
- [ ] `zIndex(int)`
- [ ] `overflowHidden()`, `overflowScroll()`

### Styling (Comprehensive)
- [ ] `border(width)`
- [ ] `borderColor(color)`
- [ ] `rounded(px)` - Border radius
- [ ] `shadow(size)`
- [ ] `opacity(float)`

### Text Formatting
- [ ] `fontSize(px)`
- [ ] `fontWeight(weight)`
- [ ] `fontFamily(name)`
- [ ] `lineHeight(px)`

### Form Elements
- [ ] Input / Text Entry
- [ ] Checkbox
- [ ] Canvas / Custom Paint

### Window Management
- [ ] Set Window Title
- [ ] Resize Window
- [ ] Quit Application
