PicoModal [![Build Status](https://secure.travis-ci.org/Nycto/PicoModal.png?branch=master)](http://travis-ci.org/Nycto/PicoModal) [![Bower version](https://badge.fury.io/bo/picoModal.png)](http://badge.fury.io/bo/picoModal) [![npm version](https://badge.fury.io/js/picomodal.svg)](https://www.npmjs.com/package/picomodal)
=========

A small, self-contained JavaScript modal library. Plain, vanilla JS.

* Small: At around 2kb minified & gzipped, it's small and easily embeddable
* No Dependencies: PicoModal does not depend on any other JS libraries,
  so you can use it in places where you don't have access to one
* Self-contained: No extra CSS or images required; just the JS
* Simple: The interface is straight forward and easy to use
* Customizable: By changing a few settings you can customize or completely
  replace the default styles and behaviour
* Accessible: Handles focus management, keyboard events and Aria tags

Download
--------

The latest version of PicoModal is available here:
[Download](https://github.com/Nycto/PicoModal/releases)

Browser Support
---------------

![Browser Support Matrix](https://saucelabs.com/browser-matrix/picomodal.svg?auth=97d19e9c77a6980f249c9e6cf7ab78ec)

Basic Example
-------------

If all you want to do is display a modal, it's as easy as this:
([Run this code](http://jsfiddle.net/nnrXD/))

```javascript
  picoModal("Ah, the pitter patter of tiny feet in huge combat boots.").show();
```

If you plan on showing the same modal multiple times, make sure you keep a
reference to the instance, like this:
([Run this code](http://jsfiddle.net/2s95h/))

```javascript
  var modal = picoModal("Ah, the pitter patter of tiny feet in huge combat boots.");
  document.getElementById("modal").addEventListener("click", function(){
      modal.show();
  });
```

For more control over the behaviour of the modal, you can pass in a settings
object: ([Run this code](http://jsfiddle.net/px7LU/))

```javascript
  picoModal({
      content: "Ah, the pitter patter of tiny feet in huge combat boots.",
      overlayStyles: {
          backgroundColor: "#169",
          opacity: 0.75
      }
  }).show();
```

A full list of settings is documented below.

Manually Closing a Modal
------------------------

If you want to programatically close the modal you can do it like this:
([Run this code](http://jsfiddle.net/KkV27/))

```javascript
  var modal = picoModal(
      "<p>Ah, the pitter patter of tiny feet in huge combat boots.<p>"
      + "<p><a href='#' class='dismiss'>Dismiss</a></p>"
  ).show();

  document.body.addEventListener('click', function(event) {
    if( /\bdismiss\b/.test(event.target.className) ) {
      modal.close();
    }
  });
```

Or you can use a more targetted implementation with the `afterCreate` event:
([Run this code](http://jsfiddle.net/GZb2z/))

```javascript
  picoModal(
      "<p>Ah, the pitter patter of tiny feet in huge combat boots.<p>"
      + "<p><a href='#' class='dismiss'>Dismiss</a></p>"
  ).afterCreate(function(modal){
      modal.modalElem().getElementsByClassName("dismiss")[0]
          .addEventListener('click', modal.close);
  }).show();
```

You can also attach an event to fire when the modal is closed:
([Run this code](http://jsfiddle.net/Lkx5r/))

```javascript
  picoModal("Ah, the pitter patter of tiny feet in huge combat boots.")
      .afterClose(function () { alert("Closed"); })
      .show();
```

Customizing Behavior
--------------------

To disable the close button, and instead just rely on someone clicking
outside of the modal, you can do this:
([Run this code](http://jsfiddle.net/WFqz4/))

```javascript
  picoModal({
      content: "Ah, the pitter patter of tiny feet in huge combat boots.",
      closeButton: false
  }).show();
```

Or, to disable closing when someone clicks outside of the modal, you can
do this: ([Run this code](http://jsfiddle.net/7d53v/))

```javascript
  picoModal({
      content: "Ah, the pitter patter of tiny feet in huge combat boots.",
      overlayClose: false
  }).show();
```

To use custom HTML for the close button, do this:
([Run this code](http://jsfiddle.net/UvcDL/))

```javascript
  picoModal({
      content: "Ah, the pitter patter of tiny feet in huge combat boots.",
      closeHtml: "<span>Close</span>",
      closeStyles: {
          position: "absolute", top: "-10px", right: "-10px",
          background: "#eee", padding: "5px 10px", cursor: "pointer",
          borderRadius: "5px", border: "1px solid #ccc"
      }
  }).show();
```

Events
------

There are a few events you can hook into for watching and sometimes monitoring
the behavior of a modal. The events are:

* `afterCreate`: Triggered when the DOM Nodes for a modal are created
* `beforeShow`: Triggered before a modal is shown. Allows for cancellation
* `afterShow`: Triggered after a modal is shown
* `beforeClose`: Triggered before a modal is closed. Allows for cancellation
* `afterClose`: triggered after a modal is closed

These exist as methods on the PicoModal instance. You can use them like this:
([Run this code](http://jsfiddle.net/875Tx/))

```javascript
  picoModal("Ah, the pitter patter of tiny feet in huge combat boots.")
      .afterClose(function (modal) {
          alert("Modal Closed: " + modal.modalElem().innerText);
      })
      .show();
```

The first argument passed to the callback is the PicoModal instance for the
specific modal.

For two of the events, beforeShow and beforeClose, there is a second argument
passed that lets you cancel the behavior in question. For example:
([Run this code](http://jsfiddle.net/m83nW/))

```javascript
  picoModal("Ah, the pitter patter of tiny feet in huge combat boots.")
      .beforeShow(function (modal, event) {
          if ( !confirm("Are you sure you want to open this modal?") ) {
              event.preventDefault();
          }
      })
      .show();
```

Single Shot Modal
-----------------

You can use the `afterClose` event and the `destroy` method to create a modal
that will clean up after itself when it is closed, like this:
([Run this code](http://jsfiddle.net/wxZ56/))

```javascript
  picoModal("Ah, the pitter patter of tiny feet in huge combat boots.")
      .afterClose(function (modal) { modal.destroy(); })
      .show();
```

Dialog
------

There is no built in dialog option, but there are tools to make it easy to
implement one yourself. If you think about it, dialogs are really just modals
that have some sort of 'result'. To achieve this, add click handlers that send
data to the `afterClose` function indicating the result:
([Run this code](http://jsfiddle.net/sLrkdjhL/))

```javascript
picoModal({
    content: "<p>Ah, the pitter patter of tiny feet in huge combat boots.</p>" +
        "<p class='footer'>" +
        "<button class='cancel'>Cancel</button> " +
        "<button class='ok'>Ok</button>" +
        "</p>"
}).afterCreate(modal => {
    modal.modalElem().addEventListener("click", evt => {
        if (evt.target && evt.target.matches(".ok")) {
            modal.close(true);
        } else if (evt.target && evt.target.matches(".cancel")) {
            modal.close();
        }
    });
}).afterClose((modal, event) => {
    alert(event.detail ? "Ok" : "Cancelled");
}).show();
```

In the example above, notice the argument passed to `modal.close()` above, and
then accessing it by reading `event.detail`.

Animation
---------

PicoModal doesn't have any built in animations, but you can use the event
system to add some of your own. For example, the following snippet adds a fade
in and out using jQuery: ([Run this code](http://jsfiddle.net/Sr697/))

```javascript
  picoModal({
      content: "Ah, the pitter patter of tiny feet in huge combat boots.",
      overlayStyles: function ( styles ) { styles.opacity = 0; },
      modalStyles: function ( styles ) { styles.opacity = 0; }
  })
  .afterShow(function(modal){
      $(modal.overlayElem()).animate({opacity: .5});
      $(modal.modalElem()).animate({opacity: 1});
  })
  .beforeClose(function(modal, event) {
      event.preventDefault();
      $(modal.overlayElem()).add(modal.modalElem())
          .animate(
              { opacity: 0 },
              { complete: modal.forceClose }
          );
  })
  .show();
```

Settings
--------

The following settings are available when creating a modal:

* __content__: The data to display to the user
* __width__: The forced width of the modal
* __closeButton__: Boolean whether to display the close button
* __closeHtml__: Custom HTML content for the close button
* __closeStyles__: A hash of CSS properties to apply to the close button
* __closeClass__: A class to attach to the close button
* __overlayClose__: Boolean whether a click on the shadow should close the modal
* __overlayStyles__: A hash of additional CSS properties to apply to the
  overlay behind the modal
* __overlayClass__: A class to attach to the overlay element
* __modalStyles__: A hash of additional CSS properties to apply to the
  modal element
* __modalClass__: A class to attach to the main modal element
* __modalId__: The ID to assign to the modal element. A default ID is generated
  used if none is provided.
* __parent__: By default, the modal dialog elements are attached to
  `document.body`. This options allows you to select an alternative parent
  element by specifying a node or a selector
* __escCloses__: When `false`, disables pressing the escape key to close this
  modal. This defaults to `true`.
* __focus__: Whether to automatically set focus on the first focusable element
  within this modal when it opens. This defaults to `true`.
* __ariaDescribedBy__: The id of the element that contains the main content
  of this modal. This sets the `[aria-describedby](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/ARIA_Techniques/Using_the_aria-describedby_attribute)`
  attribute. This defaults to the ID of the modal if none is provided.
* __ariaLabelledBy__: The id of the element that contains the general label
  for this modal. This sets the `[aria-labelledby](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/ARIA_Techniques/Using_the_aria-labelledby_attribute)`
  attributed. It is left blank if none is provided.
* __bodyOverflow__: Whether to set `overflow: hidden` on the body when the modal
  is displayed. This prevents the main page from scrolling when a modal is open

If a method is passed as an argument for any of the settings, it will be
called. The first argument passed in is the default value for that setting. This
makes it easy to modify the defaults instead of having to totally define your
own, like so: ([Run this code](http://jsfiddle.net/rptf6/))

```javascript
  picoModal({
      content: "Ah, the pitter patter of tiny feet in huge combat boots.",
      overlayStyles: function (styles) {
          styles.opacity = 0.1;
          return styles;
      }
  }).show();
```

Modal Instance API
------------------

The following methods are available on the object returned by `picoModal`:

* __modalElem__: Returns the modal DOM Node
* __closeElem__: Returns the close button DOM Node
* __overlayElem__: Returns the overlay DOM Node
* __show__: Displays the modal
* __buildDom__: Builds the DOM for the modal, but without showing it
* __close__: Hides the modal
* __forceClose__: Hides the modal without calling the beforeClose events
* __destroy__: Detaches all DOM Nodes and unhooks this modal
* __isVisible__: Whether this modal is currently being displayed
* __options__: Updates the options for this modal. This will only let you
  change options that are re-evaluted regularly, such as `overlayClose`.
* __afterCreate__: Registers a callback to invoke when the modal is created
* __beforeShow__: Registers a callback to invoke before the modal is shown
* __afterShow__: Registers a callback to invoke when the modal is shown
* __beforeClose__: Registers a callback to invoke before the modal is closed
* __afterClose__: Registers a callback to invoke when the modal is closed

License
-------

PicoModal is released under the MIT License, which is pretty spiffy. You should
have received a copy of the MIT License along with this program. If not, see
http://www.opensource.org/licenses/mit-license.php

