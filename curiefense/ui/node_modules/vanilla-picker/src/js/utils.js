class EventBucket {
    constructor() {
        this._events = [];
    }

    add(target, type, handler) {
        target.addEventListener(type, handler, false);
        this._events.push({
            target,
            type,
            handler,
        });
    }
    
    remove(target, type, handler) {
        this._events = this._events.filter(e => {
            let isMatch = true;
            if(target  && (target  !== e.target))  { isMatch = false; }
            if(type    && (type    !== e.type))    { isMatch = false; }
            if(handler && (handler !== e.handler)) { isMatch = false; }

            if(isMatch) {
                EventBucket._doRemove(e.target, e.type, e.handler);
            }
            return !isMatch;
        });
    }
    static _doRemove(target, type, handler) {
        target.removeEventListener(type, handler, false);
    }
    
    destroy() {
        this._events.forEach(e => EventBucket._doRemove(e.target, e.type, e.handler));
        this._events = [];
    }
}

function parseHTML(htmlString) {
    //https://stackoverflow.com/questions/494143/creating-a-new-dom-element-from-an-html-string-using-built-in-dom-methods-or-pro
    const div = document.createElement('div');
    div.innerHTML = htmlString;
    return div.firstElementChild;
}

function dragTrack(eventBucket, area, callback) {
    var dragging = false;

    function clamp(val, min, max) {
        return Math.max(min, Math.min(val, max));
    }

    function onMove(e, info, starting) {
        if (starting) { dragging = true; }
        if (!dragging) { return; }

        e.preventDefault();

        var bounds = area.getBoundingClientRect(),
            w = bounds.width,
            h = bounds.height,
            x = info.clientX,
            y = info.clientY;

        var relX = clamp(x - bounds.left, 0, w),
            relY = clamp(y - bounds.top, 0, h);

        callback(relX / w, relY / h);
    }

    function onMouse(e, starting) {
        var button = (e.buttons === undefined) ? e.which : e.buttons;
        if (button === 1) {
            onMove(e, e, starting);
        }
        //`mouseup` outside of window:
        else {
            dragging = false;
        }
    }

    function onTouch(e, starting) {
        if (e.touches.length === 1) {
            onMove(e, e.touches[0], starting);
        }
        //Don't interfere with pinch-to-zoom etc:
        else {
            dragging = false;
        }
    }

    //Notice how we must listen on the whole window to really keep track of mouse movements,
    //while touch movements "stick" to the original target from `touchstart` (which works well for our purposes here):
    //
    //  https://stackoverflow.com/a/51750458/1869660
    //  "Mouse moves = *hover* like behavior. Touch moves = *drags* like behavior"
    //
    eventBucket.add(area,   'mousedown',   function(e) { onMouse(e, true); });
    eventBucket.add(area,   'touchstart',  function(e) { onTouch(e, true); });
    eventBucket.add(window, 'mousemove',   onMouse);
    eventBucket.add(area,   'touchmove',   onTouch);
    eventBucket.add(window, 'mouseup',     function(e) { dragging = false; });
    eventBucket.add(area,   'touchend',    function(e) { dragging = false; });
    eventBucket.add(area,   'touchcancel', function(e) { dragging = false; });
}


export { EventBucket, parseHTML, dragTrack };
