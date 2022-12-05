# color-conversion

JavaScript library for color conversion and manipulation with support for CSS color syntax.

Demo: https://rawgit.com/Sphinxxxx/color-conversion/master/demo/index.html

## Usage

```js
<script src="https://unpkg.com/@sphinxxxx/color-conversion@2"></script>

<script>
    //rgba/hsla/hex string, CSS color name, or rgba array:
    var color = new Color('rgb(200, 250, 0)');
    
    color.hsla;  // [0.2, 1, 0.49, 1]
    color.rgba;  // [200, 250, 0, 1]
    
    color.hslaString;  // "hsla(72,100%,49.02%,1)"
    color.rgbaString;  // "rgba(200,250,0,1)"
    color.hex;         // "#c8fa00ff"
</script>
```
