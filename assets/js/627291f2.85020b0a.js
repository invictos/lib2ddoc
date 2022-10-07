"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[2634],{3905:function(e,n,t){t.d(n,{Zo:function(){return c},kt:function(){return f}});var r=t(7294);function a(e,n,t){return n in e?Object.defineProperty(e,n,{value:t,enumerable:!0,configurable:!0,writable:!0}):e[n]=t,e}function i(e,n){var t=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);n&&(r=r.filter((function(n){return Object.getOwnPropertyDescriptor(e,n).enumerable}))),t.push.apply(t,r)}return t}function o(e){for(var n=1;n<arguments.length;n++){var t=null!=arguments[n]?arguments[n]:{};n%2?i(Object(t),!0).forEach((function(n){a(e,n,t[n])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(t)):i(Object(t)).forEach((function(n){Object.defineProperty(e,n,Object.getOwnPropertyDescriptor(t,n))}))}return e}function s(e,n){if(null==e)return{};var t,r,a=function(e,n){if(null==e)return{};var t,r,a={},i=Object.keys(e);for(r=0;r<i.length;r++)t=i[r],n.indexOf(t)>=0||(a[t]=e[t]);return a}(e,n);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(r=0;r<i.length;r++)t=i[r],n.indexOf(t)>=0||Object.prototype.propertyIsEnumerable.call(e,t)&&(a[t]=e[t])}return a}var l=r.createContext({}),u=function(e){var n=r.useContext(l),t=n;return e&&(t="function"==typeof e?e(n):o(o({},n),e)),t},c=function(e){var n=u(e.components);return r.createElement(l.Provider,{value:n},e.children)},d={inlineCode:"code",wrapper:function(e){var n=e.children;return r.createElement(r.Fragment,{},n)}},p=r.forwardRef((function(e,n){var t=e.components,a=e.mdxType,i=e.originalType,l=e.parentName,c=s(e,["components","mdxType","originalType","parentName"]),p=u(t),f=a,m=p["".concat(l,".").concat(f)]||p[f]||d[f]||i;return t?r.createElement(m,o(o({ref:n},c),{},{components:t})):r.createElement(m,o({ref:n},c))}));function f(e,n){var t=arguments,a=n&&n.mdxType;if("string"==typeof e||a){var i=t.length,o=new Array(i);o[0]=p;var s={};for(var l in n)hasOwnProperty.call(n,l)&&(s[l]=n[l]);s.originalType=e,s.mdxType="string"==typeof e?e:a,o[1]=s;for(var u=2;u<i;u++)o[u]=t[u];return r.createElement.apply(null,o)}return r.createElement.apply(null,t)}p.displayName="MDXCreateElement"},3117:function(e,n,t){t.r(n),t.d(n,{frontMatter:function(){return s},contentTitle:function(){return l},metadata:function(){return u},toc:function(){return c},default:function(){return p}});var r=t(7462),a=t(3366),i=(t(7294),t(3905)),o=["components"],s={},l="\ud83d\udce8 Zone de Message",u={unversionedId:"Specifications/Message",id:"Specifications/Message",isDocsHomePage:!1,title:"\ud83d\udce8 Zone de Message",description:"La zone de message contient toutes les donn\xe9es et l\u2019information encod\xe9e. Chaque donn\xe9e est  pr\xe9c\xe9d\xe9e d\u2019un identifiant de donn\xe9e (ID). L\u2019ID de la donn\xe9e est n\xe9cessaire pour pr\xe9ciser si la donn\xe9e est de longueur fixe, variable, born\xe9 inf\xe9rieurement et/ou sup\xe9rieurement.",source:"@site/docs/Specifications/Message.md",sourceDirName:"Specifications",slug:"/Specifications/Message",permalink:"/lib2ddoc/docs/Specifications/Message",editUrl:"https://gitlab.insa-rouen.fr/acamusat/lib2ddoc/-/edit/SiteWeb/website/docs/Specifications/Message.md",tags:[],version:"current",frontMatter:{},sidebar:"docs",previous:{title:"\ud83c\udfa9 En-t\xeate",permalink:"/lib2ddoc/docs/Specifications/Entete"},next:{title:"\ud83d\udd11 Signature",permalink:"/lib2ddoc/docs/Specifications/Signature"}},c=[{value:"\ud83d\udd20 Zone de message C40",id:"-zone-de-message-c40",children:[],level:2},{value:"\ud83d\udd22 Zone de message Binaire",id:"-zone-de-message-binaire",children:[],level:2}],d={toc:c};function p(e){var n=e.components,s=(0,a.Z)(e,o);return(0,i.kt)("wrapper",(0,r.Z)({},d,s,{components:n,mdxType:"MDXLayout"}),(0,i.kt)("h1",{id:"-zone-de-message"},"\ud83d\udce8 Zone de Message"),(0,i.kt)("p",null,"La zone de message contient toutes les donn\xe9es et l\u2019information encod\xe9e. Chaque donn\xe9e est  pr\xe9c\xe9d\xe9e d\u2019un identifiant de donn\xe9e (",(0,i.kt)("strong",{parentName:"p"},"ID"),"). L\u2019ID de la donn\xe9e est n\xe9cessaire pour pr\xe9ciser si la donn\xe9e est de longueur fixe, variable, born\xe9 inf\xe9rieurement et/ou sup\xe9rieurement.  "),(0,i.kt)("p",null,"La zone de message diff\xe8re l\xe9g\xe8rement selon le format d\u2019encodage du 2D-Doc (C40 ou Binaire) et le nombre de champs peut \xe9galement varier en fonction des donn\xe9es facultatives.\nOn peut encoder diff\xe9rents types de donn\xe9es:"),(0,i.kt)("ul",null,(0,i.kt)("li",{parentName:"ul"},"Pour le format C40 cela comprend des donn\xe9es ",(0,i.kt)("strong",{parentName:"li"},"textuelles, num\xe9riques ou des dates/heures"),"."),(0,i.kt)("li",{parentName:"ul"},"Pour le format Binaire on peut encoder des ",(0,i.kt)("strong",{parentName:"li"},"donn\xe9es binaires")," ainsi que des r\xe9f\xe9rences \xe0 des donn\xe9es externes au Code 2D-DOC. ")),(0,i.kt)("h2",{id:"-zone-de-message-c40"},"\ud83d\udd20 Zone de message C40"),(0,i.kt)("p",null,"En C40, l\u2019en-t\xeate a une taille fixe de 26 caract\xe8res alphanum\xe9riques avant encodage. Un message est constitu\xe9 d\u2019une suite de blocs de donn\xe9es. Les blocs de donn\xe9es contiennent les \xe9l\xe9ments suivants: "),(0,i.kt)("ul",null,(0,i.kt)("li",{parentName:"ul"},"un identifiant de donn\xe9e ",(0,i.kt)("strong",{parentName:"li"},"ID")," cod\xe9 sur deux caract\xe8res"),(0,i.kt)("li",{parentName:"ul"},"la ",(0,i.kt)("strong",{parentName:"li"},"donn\xe9e")," elle-m\xeame encod\xe9e au format C40"),(0,i.kt)("li",{parentName:"ul"},"un caract\xe8re de fin de donn\xe9e appel\xe9 ",(0,i.kt)("strong",{parentName:"li"},"s\xe9parateur")," (",(0,i.kt)("inlineCode",{parentName:"li"},"<GS>"),") lorsqu\u2019un bloc de donn\xe9es n\u2019a pas une taille fixe")),(0,i.kt)("p",null,"Finalement, le caract\xe8re ",(0,i.kt)("inlineCode",{parentName:"p"},"<RS>")," indique la ",(0,i.kt)("strong",{parentName:"p"},"fin")," de la zone de message"),(0,i.kt)("p",null,(0,i.kt)("img",{alt:"En-t\xeate d\u2019un code 2D-Doc",src:t(8451).Z})),(0,i.kt)("h2",{id:"-zone-de-message-binaire"},"\ud83d\udd22 Zone de message Binaire"),(0,i.kt)("p",null,"L\u2019identifiant de donn\xe9e (",(0,i.kt)("strong",{parentName:"p"},"ID"),") est cod\xe9 sur un octet (La valeur ",(0,i.kt)("inlineCode",{parentName:"p"},"0xFF")," n\u2019est pas autoris\xe9e: elle est r\xe9serv\xe9e pour indiquer le d\xe9but de la signature) puis suivi d\u2019un ou trois octets pour indiquer la taille de la donn\xe9e qui suit. Si la donn\xe9e est nulle alors l\u2019octet prend la valeur de ",(0,i.kt)("inlineCode",{parentName:"p"},"0x00")))}p.isMDXComponent=!0},8451:function(e,n,t){n.Z=t.p+"assets/images/message-fef9a1433e47ba3995563b4d6e75ff22.PNG"}}]);