(self['webpackChunkwebpack'] = self['webpackChunkwebpack'] || []).push([["main"], {
"./a.svg": function (module, exports, __webpack_require__) {
"use strict";
module.exports = "data:image/svg+xml;base64,";},
"./a.js": function (module, exports, __webpack_require__) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
Object.defineProperty(exports, "a", {
    enumerable: true,
    get: function() {
        return a;
    }
});
const a = 3;
},
"./index.js": function (module, exports, __webpack_require__) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
var _aJs = __webpack_require__("./a.js");
var _aSvg = __webpack_require__.ir(__webpack_require__("./a.svg"));
_aJs.a;
_aSvg.default;
},

},function(__webpack_require__) {
var __webpack_exports__ = __webpack_require__('./index.js');

}
]);