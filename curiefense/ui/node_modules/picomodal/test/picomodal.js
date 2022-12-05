/* jshint globalstrict: true */
/* global require */

"use strict";

var testing = require('grunt-dom-test');

testing.a("modal")
    .withFile("src/picoModal.js")
    .withUtility("node_modules/chai/chai.js")

    .setup(function ($) {
        $.assert = $.window.chai.assert;
        $.picoModal = $.window.picoModal;
    })

    .should("Be created when 'show' is called").in(function (done, $) {
        $.picoModal(
            "<div>Curse your sudden but inevitable betrayal!</div>"
        ).show();
        $.assert.equal(
            $.query(".pico-content div").one().text(),
            "Curse your sudden but inevitable betrayal!");
        done();
    })

    .should("Be creatable with existing DOM content").using(
        "<div id='myContent'>Curse your sudden but inevitable betrayal!</div>"
    ).in(function (done, $) {

        $.picoModal( $.query("#myContent").one().elem ).show();

        $.assert.equal(
            $.query(".pico-content #myContent").one().text(),
            "Curse your sudden but inevitable betrayal!" );

        $.query("#myContent").one();

        done();
    })

    .should("Be re-usable via .show and .close").in(function (done, $) {
        var modal = $.picoModal("Curse your sudden but inevitable betrayal!");
        $.assert.isFalse( modal.isVisible() );

        modal.show();
        $.assert.isTrue( modal.isVisible() );

        modal.close();
        $.assert.isFalse( modal.isVisible() );

        modal.show();
        $.assert.isTrue( modal.isVisible() );

        done();
    })

    .should("Allow functions as options").in(function (done, $) {
        var modal = $.picoModal({
            content: "Curse your sudden but inevitable betrayal!",
            width: function (def) {
                $.assert.equal(def, 'auto');
                return 200;
            }
        }).show();

        $.assert.equal('200px', modal.modalElem().style.width);

        done();
    })

    .should("Be destroyable").in(function (done, $) {
        var modal = $.picoModal("Curse your sudden but inevitable betrayal!");
        $.assert.equal($.query(".pico-content").count, 0);

        modal.show();
        $.assert.equal($.query(".pico-content").count, 1);

        modal.close();
        $.assert.equal($.query(".pico-content").count, 1);

        modal.destroy();
        $.assert.equal($.query(".pico-content").count, 0);

        done();
    })

    .should("Allow its DOM to be prebuilt").in(function (done, $) {
        var modal = $.picoModal("Curse your sudden but inevitable betrayal!");
        $.assert.isFalse( modal.isVisible() );
        $.assert.equal($.query(".pico-content").count, 0);

        modal.buildDom();
        $.assert.isFalse( modal.isVisible() );
        $.assert.equal($.query(".pico-content").count, 1);

        done();
    })

    .should("Stay on screen when really tall").in(function (done, $) {
        var modal = $.picoModal(
            "<div style='height: 20000px;'>" +
                "Curse your sudden but inevitable betrayal!" +
            "</div>"
        ).show();

        var rect = modal.modalElem().getBoundingClientRect();
        $.assert.isAbove(rect.top, -1);
        $.assert.isBelow(rect.bottom, $.window.innerHeight + 1);

        done();
    })


    .and("A modal on a tall page")

    .should("Prevent the page from scrolling").using(
        "<div style='height: 20000px;'>Content!</div>"
    ).in(function (done, $) {

        // Note: I don't love this test. It feels like it is testing the
        // implementation versus that actual intention. However, I can't find
        // a good way to determine whether a page is "scrollable"

        $.assert.isFalse( !!$.document.body.style.overflow );

        var modal = $.picoModal({
            content: "Curse your sudden but inevitable betrayal!"
        }).show();

        $.assert.equal( $.document.body.style.overflow, "hidden" );

        modal.close();

        $.assert.isFalse( !!$.document.body.style.overflow );

        done();
    })

    .should("Not change the body overflow when disabled").using(
        "<div style='height: 20000px;'>Content!</div>"
    ).in(function (done, $) {

        $.picoModal({
            content: "Curse your sudden but inevitable betrayal!",
            bodyOverflow: false
        }).show();

        $.assert.isFalse( !!$.document.body.style.overflow );

        done();
    })


    .and("Modal styling")

    .should("Allow a specific width to be set").in(function (done, $) {
        var modal = $.picoModal({
            content: "Curse your sudden but inevitable betrayal!",
            width: 150
        }).show();

        var width =
            modal.modalElem().getBoundingClientRect().width ||
            modal.modalElem().style.width.replace(/[^0-9]/g, "");

        $.assert.equal( width, 150 );

        done();
    })

    .should("Allow custom classes to be added").in(function (done, $) {
        var modal = $.picoModal({
            content: "Curse your sudden but inevitable betrayal!",
            modalClass: "my-modal",
            overlayClass: "my-overlay",
            closeClass: "my-close"
        }).show();

        $.assert.equal( $.query(".my-modal").one().elem, modal.modalElem());
        $.assert.equal( $.query(".my-overlay").one().elem, modal.overlayElem());
        $.assert.equal( $.query(".my-close").one().elem, modal.closeElem());

        done();
    })

    .should("Allow custom styles to be set").in(function (done, $) {
        var modal = $.picoModal({
            content: "Curse your sudden but inevitable betrayal!",
            modalStyles: { border: "1px solid red", background: "orange" },
            overlayStyles: { border: "1px solid yellow", background: "green" },
            closeStyles: { border: "1px solid blue", background: "purple" }
        }).show();

        $.assert.equal(modal.modalElem().style.borderColor, "red");
        $.assert.equal(modal.modalElem().style.backgroundColor, "orange");

        $.assert.equal(modal.overlayElem().style.borderColor, "yellow");
        $.assert.equal(modal.overlayElem().style.backgroundColor, "green");

        $.assert.equal(modal.closeElem().style.borderColor, "blue");
        $.assert.equal(modal.closeElem().style.backgroundColor, "purple");

        done();
    })

    .should("Allow styles to be modified").in(function (done, $) {
        var modal = $.picoModal({
            content: "Curse your sudden but inevitable betrayal!",
            modalStyles: function (styles) {
                styles.border = "1px solid red";
                styles.background = "orange";
            }
        }).show();

        $.assert.equal(modal.modalElem().style.borderColor, "red");
        $.assert.equal(modal.modalElem().style.backgroundColor, "orange");
        $.assert.equal(modal.modalElem().style.padding, "20px");

        done();
    })


    .and("The modal close button")

    .should("Be disableable").in(function (done, $) {
        $.picoModal({
            content: "Curse your sudden but inevitable betrayal!",
            closeButton: false
        }).show();

        $.assert.equal($.query("pico-close").count, 0);

        done();
    })

    .should("Support custom html").in(function (done, $) {
        $.picoModal({
            content: "Curse your sudden but inevitable betrayal!",
            closeHtml: "Close this thing!"
        }).show();

        $.assert.equal(
            $.query(".pico-close").one().text(),
            "Close this thing!");

        done();
    })


    .and("Setting an alternate modal parent")

    .should("Allow an alternate parent from a selector").using(
        "<div id='myParent'></div>"
    ).in(function (done, $) {
        var modal = $.picoModal({
            content: "Curse your sudden but inevitable betrayal!",
            parent: "#myParent"
        }).show();

        $.assert.equal(
            $.query("#myParent").one().elem,
            modal.modalElem().parentNode );

        done();
    })

    .should("Allow an alternate parent from an element").using(
        "<div id='myParent'></div>"
    ).in(function (done, $) {
        var modal = $.picoModal({
            content: "Curse your sudden but inevitable betrayal!",
            parent: $.query("#myParent").one().elem
        }).show();

        $.assert.equal(
            $.query("#myParent").one().elem,
            modal.modalElem().parentNode );

        done();
    })


    .and("The modal overlay")

    .should("Close the modal when clicked").in(function (done, $) {
        var modal = $.picoModal("Curse your sudden but inevitable betrayal!");
        modal.show();

        $.query(".pico-overlay").one().click();
        $.assert.isFalse( modal.isVisible() );

        done();
    })

    .should("Allow shadow clicks to be disabled").in(function (done, $) {
        var modal = $.picoModal({
            content: "Curse your sudden but inevitable betrayal!",
            overlayClose: false
        });
        modal.show();

        $.query(".pico-overlay").one().click();
        $.assert.isTrue( modal.isVisible() );

        done();
    })


    .and("Modal events")

    .should("be triggered").in(function (done, $) {
        var modal = $.picoModal("Curse your sudden but inevitable betrayal!");

        var status = "new";

        function checkStatus(previous, next, isVisible) {
            return function (m) {
                $.assert.equal(status, previous);
                $.assert.equal(m, modal);
                $.assert.equal( modal.isVisible(), isVisible );
                status = next;
            };
        }

        modal
            .beforeShow(checkStatus("new", "beforeShow", false))
            .afterCreate(checkStatus("beforeShow", "afterCreate", false))
            .afterShow(checkStatus("afterCreate", "afterShow", true))
            .beforeClose(checkStatus("open", "beforeClose", true))
            .afterClose(checkStatus("beforeClose", "afterClose", false));

        modal.show();
        $.assert.equal(status, "afterShow");
        status = "open";

        modal.close();
        $.assert.equal(status, "afterClose");

        done();
    })

    .should("Trigger afterCreate from buildDom").in(function (done, $) {
        var modal = $.picoModal("Curse your sudden but inevitable betrayal!");
        modal
            .afterCreate(done)
            .beforeShow( $.assert.fail )
            .afterShow( $.assert.fail );
        modal.buildDom();
    })

    .should("Allow `beforeShow` to prevent show").in(function (done, $) {
        var modal = $.picoModal("Curse your sudden but inevitable betrayal!");

        modal
            .beforeShow(function (m, event) {
                event.preventDefault();
            })
            .afterCreate( $.assert.fail )
            .afterShow( $.assert.fail );

        $.assert.isFalse( modal.isVisible() );
        done();
    })

    .should("Allow `beforeClose` to prevent close").in(function (done, $) {
        var modal = $.picoModal("Curse your sudden but inevitable betrayal!");

        modal
            .beforeClose(function (m, event) {
                event.preventDefault();
            })
            .afterClose( $.assert.fail )
            .show()
            .close();

        $.assert.isTrue( modal.isVisible() );
        done();
    })

    .should("Bypass `beforeClose` with `forceClose`").in(function (done, $) {
        var modal = $.picoModal("Curse your sudden but inevitable betrayal!");

        modal
            .beforeClose( $.assert.fail )
            .afterClose(function () {
                $.assert.isFalse( modal.isVisible() );
                done();
            })
            .show()
            .forceClose();
    })

    .should("Pass arguments from `modal.buildDom` to afterCreate").in(function (done, $) {
        var modal = $.picoModal("Curse your sudden but inevitable betrayal!");
        var data = { data: "memo" };
        var count = 0;

        modal
            .afterCreate(function (iface, event) {
                count++;
                $.assert.isFunction(event.preventDefault);
                $.assert.strictEqual(event.detail, data);
                $.assert.strictEqual(iface, modal);
            })
            .buildDom(data);

        $.assert.strictEqual(count, 1);
        done();
    })

    .should("Pass arguments from `modal.show` to event handlers").in(function (done, $) {
        var modal = $.picoModal("Curse your sudden but inevitable betrayal!");
        var data = { data: "memo" };
        var count = 0;

        modal
            .beforeShow(function (iface, event) {
                count++;
                $.assert.isFunction(event.preventDefault);
                $.assert.strictEqual(event.detail, data);
                $.assert.strictEqual(iface, modal);
            })
            .afterShow(function (iface, event) {
                count++;
                $.assert.isFunction(event.preventDefault);
                $.assert.strictEqual(event.detail, data);
                $.assert.strictEqual(iface, modal);
            })
            .show(data);

        $.assert.strictEqual(count, 2);
        done();
    })

    .should("Pass arguments from `modal.close` to event handlers").in(function (done, $) {
        var modal = $.picoModal("Curse your sudden but inevitable betrayal!");
        var data = { data: "memo" };
        var count = 0;

        modal
            .beforeClose(function (iface, event) {
                count++;
                $.assert.isFunction(event.preventDefault);
                $.assert.strictEqual(event.detail, data);
                $.assert.strictEqual(iface, modal);
            })
            .afterClose(function (iface, event) {
                count++;
                $.assert.isFunction(event.preventDefault);
                $.assert.strictEqual(event.detail, data);
                $.assert.strictEqual(iface, modal);
            })
            .show()
            .close(data);

        $.assert.strictEqual(count, 2);
        done();
    })

    .should("Pass arguments from `modal.forceClose` to afterClose").in(function (done, $) {
        var modal = $.picoModal("Curse your sudden but inevitable betrayal!");
        var data = { data: "memo" };
        var count = 0;

        modal
            .afterClose(function (iface, event) {
                count++;
                $.assert.isFunction(event.preventDefault);
                $.assert.strictEqual(event.detail, data);
                $.assert.strictEqual(iface, modal);
            })
            .show()
            .forceClose(data);

        $.assert.strictEqual(count, 1);
        done();
    })


    .and("Modal accessibility")

    .should("Add an ID to every modal").in(function (done, $) {

        $.assert.equal(
            $.picoModal("Curse your sudden but inevitable betrayal!")
                .buildDom()
                .modalElem()
                .id,
            "pico-1");

        $.assert.equal(
            $.picoModal("Curse your sudden but inevitable betrayal!")
                .buildDom()
                .modalElem()
                .id,
            "pico-2");

        $.assert.equal(
            $.picoModal("Curse your sudden but inevitable betrayal!")
                .buildDom()
                .modalElem()
                .id,
            "pico-3");

        done();
    })

    .should("Allow a custom ID to be added").in(function (done, $) {
        var modal = $.picoModal({
            content: "Curse your sudden but inevitable betrayal!",
            modalId: "myModal"
        }).show();

        $.assert.equal( modal.modalElem().id, "myModal");

        done();
    })

    .should("Set aria-describedby automatically").in(function (done, $) {
        var modal = $.picoModal({
            content: "Curse your sudden but inevitable betrayal!",
            modalId: "myModal"
        }).show();

        $.assert.equal(
            modal.modalElem().getAttribute("aria-describedby"),
            "myModal");

        done();
    })

    .should("Allow custom aria attributes").in(function (done, $) {
        var modal = $.picoModal({
            content:
                "<h2 id='my-title'>Firefly Quotes</h2>" +
                "<div id='my-quote'>" +
                    "Curse your sudden but inevitable betrayal!" +
                "</div>",
            ariaLabelledBy: "my-title",
            ariaDescribedBy: "my-quote"
        }).show();

        $.assert.equal(
            modal.modalElem().getAttribute("aria-labelledby"),
            "my-title");

        $.assert.equal(
            modal.modalElem().getAttribute("aria-describedby"),
            "my-quote");

        done();
    })

    .should("Set focus on the first element").using(
        "<a id='my-link' href='#'>This is a link on the page</a>"
    ).in(function (done, $) {

        $.id("my-link").focus();
        $.assert.isTrue( $.id("my-link").isFocused() );

        var modal = $.picoModal({
            content:
                "<p>Curse your sudden but inevitable betrayal!</p>" +
                "<button id='first'></button>" +
                "<input type='text' id='second'>"
        });

        $.assert.isTrue( $.id("my-link").isFocused() );

        modal.show();
        $.assert.isTrue( $.id("first").isFocused() );

        modal.close();
        $.assert.isTrue( $.id("my-link").isFocused() );

        done();
    })

    .should("Loop the tab key back to the start").in(function (done, $) {
        var modal = $.picoModal({
            content:
                "<p>Curse your sudden but inevitable betrayal!</p>" +
                "<button id='first'></button>" +
                "<input type='text' id='second'>" +
                "<a id='third' href='#'>A link</a>" +
                "<textarea id='fourth'></textarea>"
        });

        modal.show();
        $.query(".pico-close").one().focus();
        $.assert.isTrue( $.query(".pico-close").one().isFocused() );

        $.html.keyDown(9); // 9 == tab key
        $.assert.isTrue( $.id("first").isFocused() );

        done();
    })

    .should("Loop shift-tab to the end").in(function (done, $) {
        var modal = $.picoModal({
            content:
                "<p>Curse your sudden but inevitable betrayal!</p>" +
                "<button id='first'></button>" +
                "<input type='text' id='second'>" +
                "<a id='third' href='#'>A link</a>" +
                "<textarea id='fourth'></textarea>"
        });

        modal.show();
        $.id("first").focus();
        $.assert.isTrue( $.id("first").isFocused() );

        $.html.keyDown(9, { shift: true }); // 9 == tab key
        $.assert.isTrue( $.query(".pico-close").one().isFocused() );

        done();
    })

    .should("Allow focus capture to be disabled").using(
        "<a id='my-link' href='#'>This is a link on the page</a>"
    ).in(function (done, $) {
        var modal = $.picoModal({
            content:
                "<p>Curse your sudden but inevitable betrayal!</p>" +
                "<button id='my-button'>Shiny!</button>",
            focus: false
        });

        modal.show();

        $.query(".pico-close").one().focus();
        $.assert.isTrue( $.query(".pico-close").one().isFocused() );

        $.html.keyDown(9); // 9 == tab key
        $.assert.isFalse( $.id("my-button").isFocused() );

        done();
    })

    .should("Close the modal when escape is pressed").in(function (done, $) {
        var modal = $.picoModal("Curse your sudden but inevitable betrayal!");
        modal.show();

        $.html.keyDown(27);
        $.assert.isFalse( modal.isVisible() );
        done();
    })

    .should("Allow 'escape to close' to be disabled").in(function (done, $) {
        var modal = $.picoModal({
            content: "Curse your sudden but inevitable betrayal!",
            escCloses: false
        });
        modal.show();

        $.html.keyDown(27);
        $.assert.isTrue( modal.isVisible() );
        done();
    })

    ;

