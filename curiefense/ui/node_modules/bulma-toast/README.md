# bulma-toast

[![npm version](https://badge.fury.io/js/bulma-toast.svg)](https://badge.fury.io/js/bulma-toast)
![Publish](https://github.com/rfoel/bulma-toast/workflows/Publish/badge.svg)

Bulma's pure JavaScript extension to display toasts. Basically a Bulma's [notification](https://bulma.io/documentation/elements/notification) implemented as a toast plugin.

[Demo](https://rfoel.github.io/bulma-toast/)

![Bulma Toast](https://raw.githubusercontent.com/rfoel/bulma-toast/master/bulma-toast.png)

## Options

The plugin comes with the following options:

- `message`: The actual message to be displayed. It can be a string, a template string, or a DOM node. [See examples](#examples). This is **required**.
- `type`: Essentially a Bulma's css class. It can be `is-primary`, `is-link`, `is-info`, `is-success`, `is-warning`, `is-danger`, or any other custom class. Default is a whitesmoke background with dark text as shown [here](https://bulma.io/documentation/elements/notification).
- `duration`: Duration of the notification in milliseconds. Default is `2000` milliseconds.
- `position`: Position where the notification will be shown. The default is `top-right`, so if you want it to be on the top-left just add `top-left` to this option. The available options are: `top-left`, `top-center`, `top-right`, `center`, `bottom-left`, `bottom-center`, and `bottom-right`.
- `dismissible`: Whether the notification will have a close button or not. Default is `false`.
- `pauseOnHover`: Pauses delay when hovering the notification. Default is `false`.
- `closeOnClick`: Dismisses the notification when clicked. Default is `true`.
- `opacity`: The notification's container opacity. Default is `1`.
- `animate`: [See here](#animate). Default is no animations.
- `appendTo`: By default, the notification will be appended to [`document.body`](https://developer.mozilla.org/en-US/docs/Web/API/Document/body). Pass a different [Node](https://developer.mozilla.org/en-US/docs/Web/API/Node) to have it appended else where in the [DOM](https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model).
- `extraClasses`: Adds classes for styling the toast notification.
## Install

```
  npm install bulma-toast
```

or

```
  yarn add bulma-toast
```

## Quick Start

1.  Link to bulma-toast.min.js

```html
<script src="bulma-toast.min.js"></script>
```

2.  Use bulma-toast to display a toast

```js
bulmaToast.toast({ message: 'Hello There' })
bulmaToast.toast({ message: 'General Kenobi', type: 'is-danger' })
```

## ES Modules

```js
// Import the toast function
import * as bulmaToast from 'bulma-toast'
// Or use
// import { toast as superToast } from 'bulma-toast'
// to rename your import

toast({
  message: 'Hello There',
  type: 'is-success',
  dismissible: true,
  animate: { in: 'fadeIn', out: 'fadeOut' },
})
```

## Default config

A simple default object to prevent errors. Your options will be merged with these and the defaults will be used if the fields are not provided.

```json
{
  "duration": 2000,
  "position": "top-right",
  "closeOnClick": true,
  "opacity": 1,
  "single": false,
  "offsetTop": 0,
  "offsetBottom": 0,
  "offsetLeft": 0,
  "offsetRight": 0
}
```

The default config can be updated using the funcion `setDefaults`. Also, it's possible to reset to the default config using `resetDefaults`

```js
bulmaToast.setDefaults({
  duration: 1000,
  position: 'top-left',
  closeOnClick: false,
})
```

## Change document context

```js
bulmaToast.setDoc(window.document)
```

This can be changed before each toast call and can be set to eny element.

## Animate

Bulma Toast supports [animate.css](https://daneden.github.io/animate.css/) (and maybe others?). You MUST include [animate.css](https://daneden.github.io/animate.css/) on your document's `<head>`

```html
<head>
  <link rel="stylesheet" href="animate.min.css" />
  <!-- or -->
  <link
    rel="stylesheet"
    href="https://cdn.jsdelivr.net/npm/animate.css@4.0.0/animate.min.css"
  />
  <!-- or -->
  <link
    rel="stylesheet"
    href="https://cdnjs.cloudflare.com/ajax/libs/animate.css/4.0.0/animate.min.css"
  />
</head>
```

Accepts a object with `in` and `out` with css classes to add animations. Using Animate.css you would pass a object like this:

```js
  {
    message: "I'm animated! Yay!",
    duration: 2000,
    position: "top-right",
    animate: { in: 'fadeIn', out: 'fadeOut' }
  }
```

_Warning_: Don't use `opacity` when using animations. Some of them use the opacity property like fade in and fade out.

## Examples

```js
import { toast } from 'bulma-toast'

toast({
  message: 'Hello There',
  type: 'is-success',
  dismissible: true,
  pauseOnHover: true,
})

toast({
  message: '<h1>LOOK HERE</h1>',
  type: 'is-danger',
  dismissible: true,
  pauseOnHover: true,
  animate: { in: 'fadeIn', out: 'fadeOut' },
})

const myMessage = `It's ${new Date().toDateString()}`

toast({
  message: myMessage,
  type: 'is-primary',
  position: 'center',
  closeOnClick: true,
  pauseOnHover: true,
  opacity: 0.8,
})

const elm = document.createElement('a')
elm.text = 'Visit my website!'
elm.href = 'https://rfoel.com'

toast({
  message: elm,
  type: 'is-warning',
  position: 'center',
  closeOnClick: true,
  pauseOnHover: true,
  animate: { in: 'fadeIn', out: 'fadeOut' },
})
```

## Contributing

Issues and pull requests are welcome.

## License

[MIT](https://github.com/rfoell/bulma-toast/blob/master/LICENSE)
