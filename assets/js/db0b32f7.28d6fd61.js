"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[6986],{3905:function(e,t,n){n.d(t,{Zo:function(){return p},kt:function(){return m}});var r=n(7294);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function c(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var l=r.createContext({}),d=function(e){var t=r.useContext(l),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},p=function(e){var t=d(e.components);return r.createElement(l.Provider,{value:t},e.children)},u={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},s=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,a=e.originalType,l=e.parentName,p=c(e,["components","mdxType","originalType","parentName"]),s=d(n),m=o,f=s["".concat(l,".").concat(m)]||s[m]||u[m]||a;return n?r.createElement(f,i(i({ref:t},p),{},{components:n})):r.createElement(f,i({ref:t},p))}));function m(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=n.length,i=new Array(a);i[0]=s;var c={};for(var l in t)hasOwnProperty.call(t,l)&&(c[l]=t[l]);c.originalType=e,c.mdxType="string"==typeof e?e:o,i[1]=c;for(var d=2;d<a;d++)i[d]=n[d];return r.createElement.apply(null,i)}return r.createElement.apply(null,n)}s.displayName="MDXCreateElement"},8940:function(e,t,n){n.r(t),n.d(t,{frontMatter:function(){return c},contentTitle:function(){return l},metadata:function(){return d},toc:function(){return p},default:function(){return s}});var r=n(7462),o=n(3366),a=(n(7294),n(3905)),i=["components"],c={},l="\u2714 Validator",d={unversionedId:"Implementation/decoder/Validator",id:"Implementation/decoder/Validator",isDocsHomePage:!1,title:"\u2714 Validator",description:"La validation effective d'un document s'effectue apr\xe8s son d\xe9codage, a l'aide des elements de Specification (voir ici)",source:"@site/docs/Implementation/decoder/Validator.md",sourceDirName:"Implementation/decoder",slug:"/Implementation/decoder/Validator",permalink:"/lib2ddoc/docs/Implementation/decoder/Validator",editUrl:"https://gitlab.insa-rouen.fr/acamusat/lib2ddoc/-/edit/SiteWeb/website/docs/Implementation/decoder/Validator.md",tags:[],version:"current",frontMatter:{},sidebar:"docs",previous:{title:"\ud83c\udfd7 Builder",permalink:"/lib2ddoc/docs/Implementation/decoder/Builder"},next:{title:"\ud83d\udd11 Signer",permalink:"/lib2ddoc/docs/Implementation/decoder/Signer"}},p=[],u={toc:p};function s(e){var t=e.components,n=(0,o.Z)(e,i);return(0,a.kt)("wrapper",(0,r.Z)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h1",{id:"-validator"},"\u2714 Validator"),(0,a.kt)("p",null,"La validation effective d'un document s'effectue apr\xe8s son d\xe9codage, a l'aide des elements de ",(0,a.kt)("strong",{parentName:"p"},"Specification")," (",(0,a.kt)("a",{parentName:"p",href:"/lib2ddoc/docs/Implementation/Specifications/Fonctions_validations"},"voir ici"),")"),(0,a.kt)("p",null,"On valide donc chaque ",(0,a.kt)("strong",{parentName:"p"},"DI")," (la FieldZone) ainsi que le ",(0,a.kt)("strong",{parentName:"p"},"Document")," lui m\xeame"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"Validity{\n    valid: fz_valid && doc_valid\n}\n")))}s.isMDXComponent=!0}}]);