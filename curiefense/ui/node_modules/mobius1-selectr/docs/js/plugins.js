// Avoid `console` errors in browsers that lack a console.
(function() {
    var method;
    var noop = function () {};
    var methods = [
        'assert', 'clear', 'count', 'debug', 'dir', 'dirxml', 'error',
        'exception', 'group', 'groupCollapsed', 'groupEnd', 'info', 'log',
        'markTimeline', 'profile', 'profileEnd', 'table', 'time', 'timeEnd',
        'timeline', 'timelineEnd', 'timeStamp', 'trace', 'warn'
    ];
    var length = methods.length;
    var console = (window.console = window.console || {});

    while (length--) {
        method = methods[length];

        // Only stub undefined methods.
        if (!console[method]) {
            console[method] = noop;
        }
    }
}());

// Plugins
(function(global) {
	'use strict';
	
	var defaultConfig = {
		size: 10,
		scrollY: true,
		scrollX: false,
		responsive: false
	};
	
	/**
	 * Attach removable event listener
	 * @param  {Object}   el       HTMLElement
	 * @param  {String}   type     Event type
	 * @param  {Function} callback Event callback
	 * @param  {Object}   scope    Function scope
	 * @return {Void}
	 */
	function on(el, type, callback) {
		el.addEventListener(type, callback, false);
	}

	/**
	 * Remove event listener
	 * @param  {Object}   el       HTMLElement
	 * @param  {String}   type     Event type
	 * @param  {Function} callback Event callback
	 * @return {Void}
	 */
	function off(el, type, callback) {
		el.removeEventListener(type, callback);
	}

	/**
	 * Iterator helper
	 * @param  {(Array|Object)}   collection Any object, array or array-like collection.
	 * @param  {Function} callback   The callback function
	 * @param  {Object}   scope      Change the value of this
	 * @return {Void}
	 */
	function each(collection, callback, scope) {
		if ("[object Object]" === Object.prototype.toString.call(collection)) {
			for (var d in collection) {
				if (Object.prototype.hasOwnProperty.call(collection, d)) {
					callback.call(scope, d, collection[d]);
				}
			}
		} else {
			for (var e = 0, f = collection.length; e < f; e++) {
				callback.call(scope, e, collection[e]);
			}
		}
	}

	/**
	 * Merge objects together into the first.
	 * @param  {Object} src   Source object
	 * @param  {Object} obj 	Object to merge into source object
	 * @return {Object}
	 */
	function extend(src, props) {
		props = props || {};
		var p;
		for (p in src) {
			if (src.hasOwnProperty(p)) {
				if (!props.hasOwnProperty(p)) {
					props[p] = src[p];
				}
			}
		}
		return props;
	}

	/**
	 * Create new element and apply propertiess and attributes
	 * @param  {String} name   The new element's nodeName
	 * @param  {Object} prop CSS properties and values
	 * @return {Object} The newly create HTMLElement
	 */
	function createElement(name, props) {
		var c = document,
			d = c.createElement(name);
		if (props && "[object Object]" === Object.prototype.toString.call(props)) {
			var e;
			for (e in props)
				if ("html" === e) d.innerHTML = props[e];
				else if ("text" === e) {
				var f = c.createTextNode(props[e]);
				d.appendChild(f);
			} else d.setAttribute(e, props[e]);
		}
		return d;
	}

	/**
	 * Emulate jQuery's css method
	 * @param  {Object} el   HTMLElement
	 * @param  {Object} prop CSS properties and values
	 * @return {Object|Void}
	 */
	function style(el, obj) {
		if ( !obj ) {
			return window.getComputedStyle(el);
		}
		if ("[object Object]" === Object.prototype.toString.call(obj)) {
			var s = "";
			each(obj, function(prop, val) {

				if ( typeof val !== "string" && prop !== "opacity" ) {
					val += "px";
				}

				s += prop + ": " + val + ";";
			});
			el.style.cssText += s;
		}
	}

	/**
	 * Find the closest matching ancestor to a given element
	 * @param  {Object} el 	HTMLElement
	 * @param  {Function} fn 	Callback
	 * @return {Boolean|Object} The matching HTMLElement or false
	 */
	function closest(el, fn) {
		return el && el !== document.body && (fn(el) ? el : closest(el.parentNode, fn));
	}	
	
	/**
	 * Get an element's DOMRect relative to the document instead of the viewport.
	 * @param  {Object} t 	HTMLElement
	 * @param  {Boolean} e 	Include margins
	 * @return {Object}   	Formatted DOMRect copy
	 */
	function rect(t, e) {
		var o = window,
			r = t.getBoundingClientRect(),
			x = o.pageXOffset,
			y = o.pageYOffset,
			m = {},
			f = "none";

		if (e) {
			var s = style(t);
			m = {
				top: parseInt(s["margin-top"], 10),
				left: parseInt(s["margin-left"], 10),
				right: parseInt(s["margin-right"], 10),
				bottom: parseInt(s["margin-bottom"], 10)
			};

			f = s.float;
		}

		return {
			w: r.width,
			h: r.height,
			x1: r.left + x,
			x2: r.right + x,
			y1: r.top + y,
			y2: r.bottom + y,
			margin: m,
			float: f
		};
	}	
	
	function debounce(a, b, c) {
		var d;
		return function() {
			var e = this,
				f = arguments,
				g = function() {
					d = null;
					if (!c) a.apply(e, f);
				},
				h = c && !d;
			clearTimeout(d);
			d = setTimeout(g, b);
			if (h) {
				a.apply(e, f);
			}
		};
	}
	
		/**
	 * requestAnimationFrame Polyfill
	 */
	var raf = window.requestAnimationFrame || (function() {
		var timeLast = 0;

		return window.webkitRequestAnimationFrame || window.mozRequestAnimationFrame || function(callback) {
			var timeCurrent = (new Date()).getTime(),
					timeDelta;

			/* Dynamically set the delay on a per-tick basis to more closely match 60fps. */
			/* Technique by Erik Moller. MIT license: https://gist.github.com/paulirish/1579671. */
			timeDelta = Math.max(0, 16 - (timeCurrent - timeLast));
			timeLast = timeCurrent + timeDelta;

			return setTimeout(function() { callback(timeCurrent + timeDelta); }, timeDelta);
		};
	})();
	
	function round(value, precision) {
		var m = Math.pow(10, precision || 0);
		return Math.round(value * m) / m;	
	}
	
	/**
	 * Get native scrollbar width
	 * @return {Number} Scrollbar width
	 */
	function getScrollBarWidth() {
		var width = 0, div = createElement("div", { class: "scrollbar-measure" });
		
		style(div, {
			width: 100,
			height: 100,
			overflow: "scroll",
			position: "absolute",
			top: -9999
		});

		document.body.appendChild(div);
		width = div.offsetWidth - div.clientWidth;
		document.body.removeChild(div);

		return width;
	}	
	
	function Scrollr(el, options) {
		this.el = el;
		
		if ( typeof el === "string" ) {
			this.el = document.querySelector(el);
		}
		
		this.config = extend(defaultConfig, options);
		
		this.render();
	}
	
	Scrollr.prototype.render = function() {
		
		var that = this;
		
		if ( this.rendered ) return false;
		
		this.size = getScrollBarWidth();
		
		this.wrapper = createElement("div", {
			class: "scrollr-wrapper"
		});		
		
		
		this.el.classList.add("scrollr-content");
		
		this.railContainer = createElement("div", {
			class: "scrollr-rails"
		});
		
		this.rails = {
			x: { node: createElement("div", { class: "scrollr-rail scrollr-rail-x" }) },
			y: { node: createElement("div", { class: "scrollr-rail scrollr-rail-y" }) }
		}
		
		this.bars = {
			x: { node: createElement("div", { class: "scrollr-bar" }) },
			y: { node: createElement("div", { class: "scrollr-bar" }) }
		}		
		
		this.rails.x.node.appendChild(this.bars.x.node);
		this.rails.y.node.appendChild(this.bars.y.node);
		
		this.railContainer.appendChild(this.rails.x.node);
		this.railContainer.appendChild(this.rails.y.node);
		
		this.el.parentNode.replaceChild(this.wrapper, this.el);
		this.wrapper.appendChild(this.el);
		this.wrapper.appendChild(this.railContainer);

		// Bind events
		this.events = {};
		this.events.move = this.move.bind(this);
		this.events.drag = this.drag.bind(this);
		this.events.stop = this.stop.bind(this);
		this.events.down = this.down.bind(this);
		this.events.update = this.update.bind(this);
		this.events.debounce = debounce(this.events.update, 50);
		
    on(this.el, "scroll", this.events.move);
		on(this.el, "mouseenter", this.events.move);		
		
		on(this.wrapper, "mousedown", this.events.down);
		on(document, 'mousemove', this.events.drag);
		on(document, 'mouseup', this.events.stop);	
		
		on(this.wrapper, "selectstart", function(e) {
			if  ( that.dragging ) {
				e.preventDefault();
			}
		});
		
		on(window, 'resize', this.events.debounce);	
		
		on(document, 'DOMContentLoaded', this.events.update);	
		
		this.update();
		
		this.rendered = true;
	};
	
	Scrollr.prototype.destroy = function() {
		
		if ( !this.rendered ) return false;
		
		this.el.classList.remove("scrollr-content");
		this.wrapper.parentNode.replaceChild(this.el, this.wrapper);
		
    off(this.el, "scroll", this.events.move);
		off(this.el, "mouseenter", this.events.move);		
		off(document, 'mousemove', this.events.drag);
		off(document, 'mouseup', this.events.stop);	
		off(window, 'resize', this.events.debounce);
		off(document, 'DOMContentLoaded', this.events.update);	
		
		this.rendered = false;
	};
	
	Scrollr.prototype.move = function() {
    var that = this;
		
		if ( !that.dragging ) {
		
			var scrollTop = that.el.scrollTop;
			var scrollLeft = that.el.scrollLeft;

			that.data.direction.x = false;
			that.data.direction.y = false;

			// Scrolling x
			if ( scrollLeft !== that.data.scrollLeft ) {
				that.data.direction.x = true;
			}

			// Scrolling y
			if ( scrollTop !== that.data.scrollTop ) {
				that.data.direction.y = true;
			}

			that.data.scrollTop = scrollTop;
			that.data.scrollLeft = scrollLeft;

			// Scrolled amounts
			that.data.scrolled.x = scrollLeft / (that.data.scrollWidth - that.data.clientWidth);
			that.data.scrolled.y = scrollTop / (that.data.scrollHeight - that.data.clientHeight);

			// Handle positions
			that.bars.x.position = that.data.scrolled.x * (that.rails.x.rect.w - that.bars.x.size);
			that.bars.y.position = that.data.scrolled.y * (that.rails.y.rect.h - that.bars.y.size);

			raf(function() {
				if(that.data.ratio.x >= 1) {

				} else {
					style(that.bars.x.node, {
						transform: "translate3d("+that.bars.x.position+"px, 0px, 0px)"
					})
				}

				if(that.data.ratio.y >= 1) {

				} else {
					style(that.bars.y.node, {
						transform: "translate3d(0px, "+that.bars.y.position+"px, 0px)"
					})
				}			
			});
		}
	};
	
	Scrollr.prototype.down = function(e) {
		
		var that = this;

		// e.preventDefault();
		
		var bar = closest(e.target, function(el) {
			return el && el.classList.contains("scrollr-bar");
		});
		
		if ( bar ) {
			var rectX = rect(that.bars.x.node);
			var rectY = rect(that.bars.y.node);
			var x = e.pageX - rectX.x1;
			var y = e.pageY - rectY.y1;

			this.data.origin = {
				x: x,
				y: y,
				bar: bar
			};

			this.dragging = true;

			return false;
		}
	};
	
	Scrollr.prototype.drag = function(e) {
		if ( this.dragging ) {
			
			e.preventDefault();
			
			var that = this;
		
			that.bars.x.position = e.pageX - that.data.origin.x;
			that.bars.y.position = e.pageY - that.data.origin.y;
			
			var x = that.bars.x.position / (that.rect.w - that.bars.x.size);
			var y = that.bars.y.position / (that.rect.h - that.bars.y.size);
			
			var scrollX = that.data.origin.bar === that.bars.x.node && x >= 0 && x <= 1;
			var scrollY = that.data.origin.bar === that.bars.y.node && y >= 0 && y <= 1;
			
			raf(function() {
				
				if ( scrollX ) {
					that.el.scrollLeft = that.data.scrollLeft = x * (that.data.scrollWidth - that.data.clientWidth);
					
					style(that.bars.x.node, {
						transform: "translate3d("+that.bars.x.position+"px, 0px, 0px)"
					});					
				}
				
				if ( scrollY ) {
					that.el.scrollTop = that.data.scrollTop = y * (that.data.scrollHeight - that.data.clientHeight);
					
					style(that.bars.y.node, {
						transform: "translate3d(0px, "+that.bars.y.position+"px, 0px)"
					});					
				}
			});

		}
	};
	
	Scrollr.prototype.stop = function(e) {
		var that = this;
		if ( that.dragging ) {
			that.dragging = false;
		}
	};
	
	Scrollr.prototype.update = function() {
		
		var s = {};
		
		this.getData();
	
		if ( this.data.scroll.x ) {
			s["height"] = "auto";
			s["max-width"] = this.rect.w + this.size;
			// s["max-height"] = this.rect.h + this.size;
			s["margin-bottom"] = -this.size;
			
			this.bars.y.size -= 10;
		}		
		
		if ( this.data.scroll.y ) {
			s["width"] = "auto";
			// s["max-width"] = this.rect.w + this.size;
			s["max-height"] = this.rect.h + this.size;
			s["margin-right"] = -this.size;
			
			this.bars.x.size -= 10;
		}
		
		this.wrapper.classList.toggle("scrollr-x", this.data.scroll.x);
		this.wrapper.classList.toggle("scrollr-y", this.data.scroll.y);
		
		style(this.el, s);		
		
		this.rails.x.rect = rect(this.rails.x.node);
		this.rails.y.rect = rect(this.rails.y.node);		
		
		style(this.bars.x.node, {
			width: this.bars.x.size
		});
		
		style(this.bars.y.node, {
			height: this.bars.y.size
		});
		
		style(this.rails.x.node, {
			opacity: this.data.scroll.x ? 1 : 0
		});			
		
		style(this.rails.y.node, {
			opacity: this.data.scroll.y ? 1 : 0
		});		
	};
	
	Scrollr.prototype.getData = function() {
		
		this.rect = rect(this.wrapper.parentNode);
		
		if ( this.config.responsive ) {
			this.rect = rect(this.wrapper.parentNode);
		}		
		
		var scrollTop = this.el.scrollTop;
		var scrollLeft = this.el.scrollLeft;
		
		var scrollHeight = this.el.scrollHeight;
		var clientHeight = round(this.rect.h);
		
		var scrollWidth = this.el.scrollWidth;
		var clientWidth = round(this.rect.w);	
		
		var scrolled = {
			x: scrollLeft / (scrollWidth - clientWidth),
			y: scrollTop / (scrollHeight - clientHeight)
		};
		
		var ratio = {
			x: clientWidth / scrollWidth,
			y: clientHeight / scrollHeight
		};
		
		var scroll = {
			x: scrollWidth > clientWidth,
			y: scrollHeight > clientHeight
		};
		
		this.data = {
			scrolled: scrolled,
			ratio: ratio,
			scrollTop: scrollTop,
			scrollLeft: scrollLeft,
			scrollHeight: scrollHeight,
			clientHeight: clientHeight,
			scrollWidth: scrollWidth,
			clientWidth: clientWidth,
			scroll: scroll,
			direction: {}
		};
		
		this.bars.x.size = round(this.data.clientWidth * (this.data.clientWidth / scrollWidth), 1);
		this.bars.y.size = round(this.data.clientHeight * (this.data.clientHeight / scrollHeight), 1);
		
		this.bars.x.position = scrolled.x * (clientWidth);
		this.bars.y.position = scrolled.y * (clientHeight);
	};
	
	global.Scrollr = Scrollr;
	
}(this));
