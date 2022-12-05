/**
 * Selectr test suite.
 *
 * Each module() should be wrapped in an immediately-invoked function expression,
 * in order to provide an isolated scope for the module.
 */
(function() {
    'use strict';

    /**
     * Returns a new generic dataset for tests.
     * @return {Array} A list of {value, text} objects
     */
    function exampleData() {
        return [
            { value: 'value-0', text: 'zero' },
            { value: 'value-1', text: 'one' },
            { value: 'value-2', text: 'two' },
            { value: 'value-3', text: 'three' },
            { value: 'value-4', text: 'four' },
            { value: 'value-5', text: 'five' },
            { value: 'value-6', text: 'six' },
            { value: 'value-7', text: 'seven' },
            { value: 'value-8', text: 'eight' },
            { value: 'value-9', text: 'nine' }
        ];
    }

    /**
     * Creates a new Selectr instance for tests.
     * Adds a __done__() method for cleanup when test is complete.
     * @param {Object} config Constructor options; uses defaultData() if options.data is empty
     * @param {HTMLSelectElement} el Base <select> element; creates a new one if empty
     * @return {Selectr} The new Selectr object
     */
    function newSelectr( config, el ) {
        el = el || document.createElement( "select" );
        if ( !(el.nodeName && el.nodeName === "SELECT") ) {
            throw "'el' must be an HTMLSelectElement";
        }
        document.body.appendChild( el );

        config = config || {};
        config.data = config.data || exampleData();

        var selectr = new Selectr( el, config );
        selectr.__done__ = function () {
            document.body.removeChild( selectr.container );
        };
        return selectr;
    }

    /**
     * Basic tests.
     */
    (function () {
        QUnit.module('General');

        QUnit.test( "instantiation", function( assert ) {
            var s = document.createElement( "select" );
            document.body.appendChild( s );
            var selectr = new Selectr( s, { data: exampleData() } );

            assert.equal( typeof selectr, "object", "can create new instance" );
            assert.equal( selectr.el, s, "instance has reference to <select> element" );
            assert.equal( s.selectr, selectr, "<select> element has reference to instance" );

            selectr.destroy();
            document.body.removeChild(s);
        });

        QUnit.test( "api methods", function( assert ) {
            var selectr = newSelectr();
            [
                "add",
                "clear",
                "close",
                "disable",
                "enable",
                "getValue",
                "open",
                "remove",
                "removeAll",
                "reset",
                "search",
                "serialize",
                "setValue",
                "toggle"
            ].forEach( function (method) {
                assert.equal( typeof selectr[method], "function", method + "() method exists" );
            });

            selectr.__done__();
        });
    })();

    /**
     * Tests for API methods.
     */
    (function () {
        QUnit.module( "API Methods" );

        QUnit.test( "setValue(), getValue()", function ( assert ) {
            var oneSelectr = newSelectr();

            assert.equal(
                oneSelectr.getValue(),
                "value-0",
                "select-one: first option is selected by default"
            );

            oneSelectr.setValue( "value-2" );
            assert.equal(
                oneSelectr.getValue(),
                "value-2",
                "select-one: sets and gets a single value"
            );

            oneSelectr.__done__();


            var multiSelectr = newSelectr( { multiple: true } );

            assert.deepEqual(
                multiSelectr.getValue(),
                [],
                "select-multi: no option is selected by default"
            );

            multiSelectr.setValue( "value-1" );
            assert.deepEqual(
                multiSelectr.getValue(),
                ["value-1"],
                "select-multi: sets and gets a single value"
            );
            assert.deepEqual(
                multiSelectr.getValue( true ),
                { values: [{ text: "one", value: "value-1" }] },
                "gets selected values in object format"
            );
            assert.deepEqual(
                JSON.parse( multiSelectr.getValue( true, true ) ),
                { values: [{ text: "one", value: "value-1" }] },
                "gets selected values as json-encoded string"
            );

            multiSelectr.setValue( "value-1" );
            assert.deepEqual(
                multiSelectr.getValue(),
                [],
                "select-multi: deselects selected values"
            );

            multiSelectr.setValue( ["value-2", "value-5"] );
            assert.deepEqual(
                multiSelectr.getValue(),
                ["value-2", "value-5"],
                "select-multi: sets and gets multiple values"
            );
            assert.deepEqual(
                multiSelectr.getValue( true ),
                { values: [{ text: "two", value: "value-2" },{ text: "five", value: "value-5" }] },
                "gets selected values in object format"
            );

            multiSelectr.__done__();
        });

        QUnit.test( "search()", function ( assert ) {
            var selectr = newSelectr();

            assert.deepEqual(
                selectr.search("one"),
                [{value: "value-1", text: "one"}],
                "matches values by display text"
            );
            assert.deepEqual(
                selectr.search("tw"),
                [{value: "value-2", text: "two"}],
                "matches partial values by display text"
            );

            selectr.input.value = "five";
            assert.deepEqual(
                selectr.search(),
                [{value: "value-5", text: "five"}],
                "searches from user input"
            );
            assert.ok( selectr.tree.querySelector( ".selectr-match" ), "tree is displayed" );
            assert.ok(
                selectr.tree.querySelector( ".selectr-match" ).textContent.trim() === "five",
                "live search results are displayed in tree"
            );

            selectr.__done__();
        });

        // @todo tests for other public API methods:
        // add
        // remove
        // removeAll
        // serialize
        // open
        // close
        // toggle
        // clear
        // reset
        // disable
        // enable
    })();

    /**
     * Tests for constructor options.
     */
    (function () {
        QUnit.module( "Constructor Options" );

        QUnit.test( "data", function ( assert ) {
            var data = exampleData();
            data[5].selected = true;
            var selectr = newSelectr( { data: data } );

            data.forEach( function ( option, index ) {
                assert.equal(
                    selectr.options[index].value,
                    option.value,
                    "option with value '" + option.value + "' exists"
                );
                assert.equal(
                    selectr.options[index].textContent,
                    option.text,
                    "option with value '" + option.value + "' has expected label '" + option.text + "'"
                );
            });
            assert.equal(
                selectr.getValue(),
                "value-5",
                "option passed with 'selected' property is selected by default"
            );

            selectr.__done__();
        });

        QUnit.test( "nativeKeyboard", function ( assert ) {
            function doKeypress( target, character ) {

                // phantomJS
                if ( typeof KeyboardEvent !== "function" ) {
                    ["keydown", "keypress", "keyup"].forEach( function (type) {
                        var event = document.createEvent( "CustomEvent" );
                        event.initCustomEvent( type, true, true );
                        event.key = character;
                        target.dispatchEvent( event );
                    });
                    return;
                }

                // modern browsers
                ["keydown", "keypress", "keyup"].forEach( function (type) {
                    target.dispatchEvent( new KeyboardEvent( type, { key: character } ) );
                });
            }

            var singleSelectr = newSelectr( { nativeKeyboard: true } );
            var multiSelectr = newSelectr( { nativeKeyboard: true, multiple: true } );
            [singleSelectr, multiSelectr].forEach( function( selectr ) {
                var subject = (selectr === singleSelectr) ? "select-one" : "select-multi";

                // when focused, [space] should toggle the dropdown.
                selectr.close();
                selectr.selected.focus();
                doKeypress( selectr.selected, " " );
                assert.ok(
                    selectr.opened,
                    subject + " toggles open on [Space] when focused and closed"
                );

                selectr.open();
                selectr.selected.focus();
                doKeypress( selectr.selected, " " );
                assert.notOk(
                    selectr.opened,
                    subject + " toggles closed on [Space] when focused and open"
                );

                // when closed + focused, [enter], [↓], [↑] should open the dropdown
                ["Enter", "ArrowDown", "ArrowUp"].forEach( function( key ) {
                    selectr.close();
                    document.activeElement.blur();
                    doKeypress( selectr.selected, key );
                    assert.notOk(
                        selectr.opened,
                        subject + " does not open on [" + key + "] when not focused"
                    );

                    selectr.close();
                    selectr.selected.focus();
                    doKeypress( selectr.selected, key );
                    assert.ok( selectr.opened, subject + " opens on [" + key + "] when focused" );

                    selectr.open();
                    selectr.selected.focus();
                    doKeypress( selectr.selected, key );
                    assert.ok( selectr.opened, subject + " does not close on [" + key + "]" );
                });

                // when open, [esc] should close the dropdown
                selectr.open();
                doKeypress( selectr.container, "Escape" );
                assert.notOk( selectr.opened, subject + " closes on [Esc] when open" );

                selectr.close();
                doKeypress( selectr.container, "Escape" );
                assert.notOk( selectr.opened, subject + " does not open on [Esc]" );


            });

            // when focused and not multiple, typing should select first matching option
            singleSelectr.selected.focus();
            doKeypress( singleSelectr.selected, "t" );
            assert.equal(
                singleSelectr.getValue(),
                "value-2",
                "select-one selects the first matching option when focused and typing"
            );

            doKeypress( singleSelectr.selected, "h" );
            assert.equal(
                singleSelectr.getValue(),
                "value-3",
                "select-one builds on existing search on additional typing"
            );

            // when focused and multiple, typing should open the dropdown and use the search feature
            multiSelectr.selected.focus();
            doKeypress( multiSelectr.selected, "e" );
            assert.ok( multiSelectr.opened, "select-multi opens when focused and typing" );
            assert.equal(
                multiSelectr.input.value,
                "e",
                "select-multi proxies search feature when focused and typing"
            );

            singleSelectr.__done__();
            multiSelectr.__done__();
        });

        QUnit.test( "defaultSelected", function ( assert ) {
             // #93 defaultSelected = false did not work as expected
             // The selectr instance with a complete select elemnt in the dom 
             // did also not give the same results as a selectr instance created with data to its constructor. 

            var oneSelectr = newSelectr({defaultSelected: false});

            assert.equal(
                oneSelectr.getValue(),
                null,
                "select-one: Default selection turned off"
            );

            oneSelectr.__done__();

            var multiSelectr = newSelectr( { defaultSelected: false, multiple: true } );
               
            assert.deepEqual(
                multiSelectr.getValue(),
                [],
                "select-multi: no option is selected by default"
            );

            multiSelectr.__done__();

            // building a selectr instance from a existing select component should have the same results 
            
            var s = document.createElement("select");
            document.body.appendChild(s);
            
            var data = exampleData();
            // Create and append the options
            for (var i = 0; i < data.length; i++) {
                var o = document.createElement("option");
                o.value = data[i].value;
                o.text = data[i].text;
                s.appendChild(o);
            }

            var oneSelectr2 = newSelectr({defaultSelected: false}, s);

            assert.equal(
                oneSelectr2.getValue(),
                null,
                "select-one: Default selection turned off"
            );

            oneSelectr2.__done__();
            
            var multiSelectr2 = newSelectr( { defaultSelected: false, multiple: true }, s);
               
            assert.deepEqual(
                multiSelectr2.getValue(),
                [],
                "select-multi: no option is selected by default"
            );

            multiSelectr2.__done__();


        });

        // @todo tests for other constructor options/settings:
        // multiple
        // searchable
        // clearable
        // allowDeselect
        // width
        // placeholder
        // maxSelections
        // taggable
        // tagSeperators
        // tagPlaceholder
        // renderOption
        // renderSelection
        // pagination
        // nativeDropdown
        // closeOnScroll
        // sortSelected
        // customClass
    })();
})();
