(self['webpackChunkwebpack'] = self['webpackChunkwebpack'] || []).push([["main"], {
"./foo.js": function (module, __webpack_exports__, __webpack_require__) {
'use strict';
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {'foo': function() { return foo; }});
var foo = "lol";
var bar = "wut";
var baz = bar || foo;

},
"./index.js": function (module, __webpack_exports__, __webpack_require__) {
'use strict';
__webpack_require__.r(__webpack_exports__);
/* harmony import */var _foo_js__WEBPACK_IMPORTED_MODULE_0_ = __webpack_require__(/* ./foo.js */"./foo.js");

console.log(_foo_js__WEBPACK_IMPORTED_MODULE_0_["foo"]);
},

},function(__webpack_require__) {
var __webpack_exec__ = function(moduleId) { return __webpack_require__(__webpack_require__.s = moduleId) }
var __webpack_exports__ = (__webpack_exec__("./index.js"));

}
]);