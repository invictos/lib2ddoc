"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[7364],{3905:function(e,t,n){n.d(t,{Zo:function(){return p},kt:function(){return m}});var r=n(7294);function i(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function o(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){i(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function c(e,t){if(null==e)return{};var n,r,i=function(e,t){if(null==e)return{};var n,r,i={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(i[n]=e[n]);return i}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(i[n]=e[n])}return i}var s=r.createContext({}),l=function(e){var t=r.useContext(s),n=t;return e&&(n="function"==typeof e?e(t):o(o({},t),e)),n},p=function(e){var t=l(e.components);return r.createElement(s.Provider,{value:t},e.children)},u={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},d=r.forwardRef((function(e,t){var n=e.components,i=e.mdxType,a=e.originalType,s=e.parentName,p=c(e,["components","mdxType","originalType","parentName"]),d=l(n),m=i,f=d["".concat(s,".").concat(m)]||d[m]||u[m]||a;return n?r.createElement(f,o(o({ref:t},p),{},{components:n})):r.createElement(f,o({ref:t},p))}));function m(e,t){var n=arguments,i=t&&t.mdxType;if("string"==typeof e||i){var a=n.length,o=new Array(a);o[0]=d;var c={};for(var s in t)hasOwnProperty.call(t,s)&&(c[s]=t[s]);c.originalType=e,c.mdxType="string"==typeof e?e:i,o[1]=c;for(var l=2;l<a;l++)o[l]=n[l];return r.createElement.apply(null,o)}return r.createElement.apply(null,n)}d.displayName="MDXCreateElement"},2631:function(e,t,n){n.r(t),n.d(t,{frontMatter:function(){return c},contentTitle:function(){return s},metadata:function(){return l},toc:function(){return p},default:function(){return d}});var r=n(7462),i=n(3366),a=(n(7294),n(3905)),o=["components"],c={},s="\ud83c\udfa9 En-t\xeate",l={unversionedId:"Specifications/Entete",id:"Specifications/Entete",isDocsHomePage:!1,title:"\ud83c\udfa9 En-t\xeate",description:"\ud83d\udcc4 Description",source:"@site/docs/Specifications/Entete.md",sourceDirName:"Specifications",slug:"/Specifications/Entete",permalink:"/lib2ddoc/docs/Specifications/Entete",editUrl:"https://gitlab.insa-rouen.fr/acamusat/lib2ddoc/-/edit/SiteWeb/website/docs/Specifications/Entete.md",tags:[],version:"current",frontMatter:{},sidebar:"docs",previous:{title:"\ud83d\udcd2 Format g\xe9n\xe9ral",permalink:"/lib2ddoc/docs/Specifications/Format_2DDoc"},next:{title:"\ud83d\udce8 Zone de Message",permalink:"/lib2ddoc/docs/Specifications/Message"}},p=[{value:"\ud83d\udcc4 Description",id:"-description",children:[],level:2},{value:"\u2692 R\xe9capitulatif",id:"-r\xe9capitulatif",children:[],level:2}],u={toc:p};function d(e){var t=e.components,c=(0,i.Z)(e,o);return(0,a.kt)("wrapper",(0,r.Z)({},u,c,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h1",{id:"-en-t\xeate"},"\ud83c\udfa9 En-t\xeate"),(0,a.kt)("h2",{id:"-description"},"\ud83d\udcc4 Description"),(0,a.kt)("p",null,"Il y a diff\xe9rentes versions d'en-t\xeate utilis\xe9es pour le dispositif 2D-DOC. L\u2019en-t\xeate contient les informations qui permettent de d\xe9coder et de v\xe9rifier les informations du 2D-DOC. La longueur de l\u2019en-t\xeate est de 22 bits pour les versions \u201801\u2019, \u201802\u2019, 24 bits pour la version \u201803\u2019 et 26 bits pour la version \u201804\u2019  car on ajoute des informations en plus dans les versions \u201803\u2019 et \u201804\u2019."),(0,a.kt)("p",null,"L\u2019en-t\xeate contient tout ce qui est reli\xe9 \xe0 l'identification du document. Ces informations permettent d\u2019interpr\xe9ter l\u2019information contenu dans un 2D-DOC car elles fournissent le type de Code 2D-DOC, la version du code, les identifiants etc."),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("p",{parentName:"li"},"Tout d\u2019abord , on retrouve dans l'en-t\xeate un ",(0,a.kt)("strong",{parentName:"p"},"marqueur")," d\u2019identification du document. Celui-ci prend toujours la valeur ",(0,a.kt)("inlineCode",{parentName:"p"},"DC"),". ")),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("p",{parentName:"li"},"Ensuite on a la ",(0,a.kt)("strong",{parentName:"p"},"version")," du 2D-DOC (eg. ",(0,a.kt)("inlineCode",{parentName:"p"},"02"),")")),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("p",{parentName:"li"},"Puis on a l\u2019",(0,a.kt)("strong",{parentName:"p"},"Identifiant de l\u2019Autorit\xe9 de Certification")," ayant d\xe9livr\xe9 le certificat utilis\xe9 par l\u2019\xe9metteur du document pour signer les Donn\xe9es. En fonction du format de codage, celui-ci peut \xeatre cod\xe9 sur 4 caract\xe8res alphanum\xe9riques (format C40) ou sur 5 caract\xe8res hexad\xe9cimaux (format BINAIRE). Un exemple d\u2019Identifiant de l\u2019Autorit\xe9 de Certification est Certigna ",(0,a.kt)("inlineCode",{parentName:"p"},"FR03")," (Service de s\xe9curit\xe9 informatique) ou Ariadnext ",(0,a.kt)("inlineCode",{parentName:"p"},"FR04")," (Entreprise de logiciels). ")),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("p",{parentName:"li"},"Apr\xe8s, on a l\u2019",(0,a.kt)("strong",{parentName:"p"},"Identifiant du Certificat")," utilis\xe9 pour signer les donn\xe9es sur quatre caract\xe8res alphanum\xe9riques. Ainsi on r\xe9cup\xe8re les certificats via des ",(0,a.kt)("em",{parentName:"p"},"Annuaire de certificats")," qui suivent la norme ",(0,a.kt)("a",{parentName:"p",href:"https://datatracker.ietf.org/doc/html/rfc4387"},"RFC 4387"),". Ce certificat contient une cl\xe9 publique qui est distribu\xe9e aux tierces parties int\xe9ress\xe9es par la v\xe9rification du code 2D-Doc. Ce certificat ne peut \xeatre stock\xe9 directement dans le 2D-DOC car il y aurait trop d\u2019informations \xe0 coder dans le code \xe0 barres et celui-ci serait beaucoup trop grand. (eg. ",(0,a.kt)("inlineCode",{parentName:"p"},"AB04"),")")),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("p",{parentName:"li"},"on a \xe9galement la ",(0,a.kt)("strong",{parentName:"p"},"date d'\xe9mission")," et la ",(0,a.kt)("strong",{parentName:"p"},"date de signature")," du document cod\xe9 en hexad\xe9cimal depuis le premier janvier 2000. Par exemple, la date du 29 avril 2021 correspond \xe0 ",(0,a.kt)("inlineCode",{parentName:"p"},"1E6D"),".")),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("p",{parentName:"li"},"Enfin, le code d\u2019identification du ",(0,a.kt)("strong",{parentName:"p"},"type de document")," (",(0,a.kt)("strong",{parentName:"p"},(0,a.kt)("a",{parentName:"strong",href:"/lib2ddoc/docs/Specifications/Types_de_documents"},"d\xe9tails")),") sur deux caract\xe8res alphanum\xe9riques."))),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("p",{parentName:"li"},"La version \u201803\u2019 du 2D-DOC se diff\xe9rencie des versions pr\xe9c\xe9dentes par l\u2019ajout d\u2019un champ pour l\u2019",(0,a.kt)("strong",{parentName:"p"},"identifiant du p\xe9rim\xe8tre")," sur lequel le type de document est d\xe9fini. Pour l'instant, l'ensemble des documents est sous l\u2019identifiant ",(0,a.kt)("inlineCode",{parentName:"p"},"01")," (p\xe9rim\xe8tre ANTS).")),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("p",{parentName:"li"},"La version \u201804\u2019 du 2D-DOC on ajoute  l\u2019information ",(0,a.kt)("strong",{parentName:"p"},"Pays \xe9metteur du document"),". Pour la France, cela correspond au code ",(0,a.kt)("inlineCode",{parentName:"p"},"FR"),"."))),(0,a.kt)("div",{className:"admonition admonition-note alert alert--secondary"},(0,a.kt)("div",{parentName:"div",className:"admonition-heading"},(0,a.kt)("h5",{parentName:"div"},(0,a.kt)("span",{parentName:"h5",className:"admonition-icon"},(0,a.kt)("svg",{parentName:"span",xmlns:"http://www.w3.org/2000/svg",width:"14",height:"16",viewBox:"0 0 14 16"},(0,a.kt)("path",{parentName:"svg",fillRule:"evenodd",d:"M6.3 5.69a.942.942 0 0 1-.28-.7c0-.28.09-.52.28-.7.19-.18.42-.28.7-.28.28 0 .52.09.7.28.18.19.28.42.28.7 0 .28-.09.52-.28.7a1 1 0 0 1-.7.3c-.28 0-.52-.11-.7-.3zM8 7.99c-.02-.25-.11-.48-.31-.69-.2-.19-.42-.3-.69-.31H6c-.27.02-.48.13-.69.31-.2.2-.3.44-.31.69h1v3c.02.27.11.5.31.69.2.2.42.31.69.31h1c.27 0 .48-.11.69-.31.2-.19.3-.42.31-.69H8V7.98v.01zM7 2.3c-3.14 0-5.7 2.54-5.7 5.68 0 3.14 2.56 5.7 5.7 5.7s5.7-2.55 5.7-5.7c0-3.15-2.56-5.69-5.7-5.69v.01zM7 .98c3.86 0 7 3.14 7 7s-3.14 7-7 7-7-3.12-7-7 3.14-7 7-7z"}))),"note")),(0,a.kt)("div",{parentName:"div",className:"admonition-content"},(0,a.kt)("p",{parentName:"div"},"On obtient un code de la forme suivante pour l\u2019en-t\xeate des versions C40 \u201801\u2019 et \u201802\u2019 :\n",(0,a.kt)("inlineCode",{parentName:"p"},"DC02ANTSXT4A1E6D1E6D01")))),(0,a.kt)("h2",{id:"-r\xe9capitulatif"},"\u2692 R\xe9capitulatif"),(0,a.kt)("p",null,"En-t\xeate d\u2019un code 2D-Doc en C40 pour la version 04 :"),(0,a.kt)("p",null,(0,a.kt)("img",{alt:"En-t\xeate d\u2019un code 2D-Doc",src:n(7581).Z})))}d.isMDXComponent=!0},7581:function(e,t,n){t.Z=n.p+"assets/images/entete-459bf75e5fea31e8a4807eb513774926.PNG"}}]);