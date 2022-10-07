"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[4922],{3905:function(e,t,n){n.d(t,{Zo:function(){return s},kt:function(){return S}});var o=n(7294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);t&&(o=o.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,o)}return n}function a(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?i(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):i(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function c(e,t){if(null==e)return{};var n,o,r=function(e,t){if(null==e)return{};var n,o,r={},i=Object.keys(e);for(o=0;o<i.length;o++)n=i[o],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(o=0;o<i.length;o++)n=i[o],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var p=o.createContext({}),l=function(e){var t=o.useContext(p),n=t;return e&&(n="function"==typeof e?e(t):a(a({},t),e)),n},s=function(e){var t=l(e.components);return o.createElement(p.Provider,{value:t},e.children)},u={inlineCode:"code",wrapper:function(e){var t=e.children;return o.createElement(o.Fragment,{},t)}},h=o.forwardRef((function(e,t){var n=e.components,r=e.mdxType,i=e.originalType,p=e.parentName,s=c(e,["components","mdxType","originalType","parentName"]),h=l(n),S=r,f=h["".concat(p,".").concat(S)]||h[S]||u[S]||i;return n?o.createElement(f,a(a({ref:t},s),{},{components:n})):o.createElement(f,a({ref:t},s))}));function S(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var i=n.length,a=new Array(i);a[0]=h;var c={};for(var p in t)hasOwnProperty.call(t,p)&&(c[p]=t[p]);c.originalType=e,c.mdxType="string"==typeof e?e:r,a[1]=c;for(var l=2;l<i;l++)a[l]=n[l];return o.createElement.apply(null,a)}return o.createElement.apply(null,n)}h.displayName="MDXCreateElement"},2362:function(e,t,n){n.r(t),n.d(t,{frontMatter:function(){return c},contentTitle:function(){return p},metadata:function(){return l},toc:function(){return s},default:function(){return h}});var o=n(7462),r=n(3366),i=(n(7294),n(3905)),a=["components"],c={},p="\ud83c\udfea SpecificationStore",l={unversionedId:"Implementation/Specifications/SpecificationStore",id:"Implementation/Specifications/SpecificationStore",isDocsHomePage:!1,title:"\ud83c\udfea SpecificationStore",description:"\ud83d\udca1 Id\xe9e",source:"@site/docs/Implementation/Specifications/SpecificationStore.md",sourceDirName:"Implementation/Specifications",slug:"/Implementation/Specifications/SpecificationStore",permalink:"/lib2ddoc/docs/Implementation/Specifications/SpecificationStore",editUrl:"https://gitlab.insa-rouen.fr/acamusat/lib2ddoc/-/edit/SiteWeb/website/docs/Implementation/Specifications/SpecificationStore.md",tags:[],version:"current",frontMatter:{},sidebar:"docs",previous:{title:"\ud83c\udfe3 ANTS -> JSON",permalink:"/lib2ddoc/docs/Implementation/Specifications/GDoc+GDoc->JSON"},next:{title:"\ud83e\udd47 Fonctions de validation",permalink:"/lib2ddoc/docs/Implementation/Specifications/Fonctions_validations"}},s=[{value:"\ud83d\udca1 Id\xe9e",id:"-id\xe9e",children:[],level:2},{value:"\ud83d\udd27 JSON -&gt; HashMap",id:"-json---hashmap",children:[],level:2},{value:"\ud83c\udfc1 Execution",id:"-execution",children:[],level:2}],u={toc:s};function h(e){var t=e.components,c=(0,r.Z)(e,a);return(0,i.kt)("wrapper",(0,o.Z)({},u,c,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("h1",{id:"-specificationstore"},"\ud83c\udfea SpecificationStore"),(0,i.kt)("h2",{id:"-id\xe9e"},"\ud83d\udca1 Id\xe9e"),(0,i.kt)("p",null,"Nous avons besoin des sp\xe9cifications \xe0 de nombreux moments de la proc\xe9dure de d\xe9codage."),(0,i.kt)("p",null,"Nous avons donc d\xe9cid\xe9 d'impl\xe9menter nos sp\xe9cifications \xe0 travers une structure suivant un pattern Singleton:\nUne hashmap est charg\xe9e durant l'initialisation, et gard\xe9e en m\xe9moire durant la vie du programme."),(0,i.kt)("p",null,"Ces structures sont ",(0,i.kt)("strong",{parentName:"p"},"FieldSpecificationStore")," et ",(0,i.kt)("strong",{parentName:"p"},"DocumentSpecificationStore"),", et impl\xe9mente le trait ",(0,i.kt)("strong",{parentName:"p"},"SpecificationStore")),(0,i.kt)("p",null,(0,i.kt)("img",{alt:"SpecificationStore",src:n(289).Z})),(0,i.kt)("h2",{id:"-json---hashmap"},"\ud83d\udd27 JSON -> HashMap"),(0,i.kt)("p",null,"C'est notre toolchain rust (build.rs) qui transforme le JSON en code rust, corps de l'impl\xe9mentation fill de ",(0,i.kt)("strong",{parentName:"p"},"SpecificationStore"),". "),(0,i.kt)("h2",{id:"-execution"},"\ud83c\udfc1 Execution"),(0,i.kt)("p",null,"Lors de l'ex\xe9cution du programme, c'est une crate rust (",(0,i.kt)("strong",{parentName:"p"},"lazy_static"),") qui cr\xe9e une r\xe9f\xe9rence accessible de mani\xe8re globale. On a donc bien une hashmap statique !"))}h.isMDXComponent=!0},289:function(e,t){t.Z="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAaQAAADDCAYAAADNymVjAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAACTwSURBVHhe7Z0JlNRUvsYRUBBUVARkX2UTEJVNAZ8KiKAgi6ICDoMsMwqigwMCKjjznsyTXd+4Igxr213d0AsMtECzCQ22NK3oKG6DI9OHpVVoQEQY/L/+bldiKpVUqppUdy3f75zvVCU3uUlu7r1fcnNzU04IIYSQCICGRAghJCKgIRFCCIkIaEiEEEIiAhoSIYSQiICGRFzh7LETcmRLLkUFrbPHT3pzDyHF0JCIK6CCSSzXiaKC1tGtud7cQ0gxNCTiCpoh7Z8/p+h/IkXZav+8OTQkYgkNibiCZkiocH75ZS9F2erw5kQaErGEhkRcgYZEBSvNkDLnLZTs7GzJy8uTL774QvLz86WwsNCbo0g8QkMirkBDooKVZkieF+ZKRkaGZGVlSU5OjjKlgoICb44i8QgNibhCKIb089kPZN+apbJu9ERJbjtQkmr1LK6gru0pqTcMkm2PT5L8ze/IL+dzLdenoluaIS2fOlMSEhIkNTVVmRLulHCXROIXGhJxhWAM6dz5vfKBZ6F4Wt0nSS36i2fcJElZ+bqkbFghqz7KUL+YxnxPy/6SXmRW+RtWWsZFRa80Q1o25UVZsmSJMiXcKaH57sCBA94cReIRGhJxBSdDOnU6RzJGTJDEBr0led4sZUBOwnLJDe+WnPGT5fw53i3FijRDSpoxRxnS0qVLxePxqLskNNuR+IWGRFwhkCHBjFJuHy6ebkNlVXaypfnYqmj5lO7DZEvfUTSlGBENidhBQyKuYGdIaKbDnZEyo7x0a9NxUtF6q28bLnsmTPGJm4pO0ZCIHTQk4gp2hoRnRmimW7UzxDsjs4rWX924jxyKoGdK+/enSrVql+nTM2c+IVdeeblUrXqpnDy502fZksq8jVgQDYnYQUMirmBlSOhN52nd3/KZUUrRXc9jM8ZLk1ZNpULFCnLxJRdL09bNpE3HttL65uv9lodSF8yWzBsH+/W+y8lZId26tZcqVSrL5ZdXlXbtrpO0tPk+y4Rbx4+/JxUqlJfPPlttGR6sPv10tYrHKuxCFSidwrlds2hIxA4aEnEFK0NC1+6kFv38jCXlw3S5vd+dUq5cOWVErW66Xlq2b6WMCfOqXFbFbx1NGW0GyJGtSfo2zpx5X2rWvFpeeeUZ+fnnHDl3bo/s3r1Mtm1bpC9TGsKdjBsVeriMwSmdLnS7Z4suPqzmW4mGROygIRFXsDKkdaP/IMnjJvmZytOzn1HG0/C6hvLWhsX6/DczF8nVNasHNKTMp6dJ3sRp+ja++mqNiuunn3br84zSKtq3354h9evXUncGY8cOUhW0tgya18aMGSTVq1dTTW6PPnqfnD79a3w//LBNhg3rq8LQfPbb3/b3ifujjzzqrgP7gea6Xr26BFwPevXVqdKgwbVqvc6d2xRVxOlqfu3a1+jxQKtWzfExioKCLXL//T3liiuqqv196qmhuhlo+/Paa9OkRo2rpF69WrJ9+2IV5pRO5u3m5iYEta2FC6dLw4a1pWfPzmq+U1pCNCRiBw2JuIKVIaUU3c2krHjDz1Ru7t5BVX4vLn3JL2zUlLHS+4E+fvM1rUt6WzZ0eEDfBq72UfEOGnSnpKcvkMOHN+lhECpObAvGcOpUthw5kiWdOrWR6dPH6ss88sg9MmRILyksfE9OnNgh997bXSZPHqGHI27o2LHtqkLOzl6qx62ZhfG/03rQhg2vqwof+4/t9+3bTc03x2Oe7tfvNnnwwbvUseBYO3RoLTNm/E5fFsf67LOj1B0Qfm+5pZ0KCyadzPsfzLYef3yIMjnNdJzSEqIhETtoSMQVrAwpqeadkrIpwc9U6jWpryozT26qX5iT0rISJb1OL58K7uuv16i7nqZN68lFF12knpNodxxaxfnNN+v05deseVmaNauv/uOKvmLFCnLwYKYevnPnEmnUqI76j2dD5cuXlwMH/q6HazJW4uYKPdB6ZqHpDHcU+G+OxziNOHF8xmPJyPj1WLRj/f77rWoadzm4I9SWdUon8/4Hsy0YvBbulJaaaEjEDhoScQUrQ0q85FZZtcffdOo0qqsqM3RsMIc5KX1vungqdfWp4Iw6dGijPPRQb/3OABUnKlbjMvv2eeSyy6qo/19+maH2BU1qmtBEddVVV6hwVNjm9TUZK3FzhR5oPejNN59TlTsMA6pc+RI13xyPcdoqTuOxWK1bqVJxvGZZpZPT/gfaFuSUlppoSMQOGhJxBStD8tTsYXmHhE4MqLheTn3VL8xJmds9fndIZuHOABUh/qPixLaMV/pr176i7hLwX+sdh18t3Cg0PeFOx7i+JmOlbK6gA62Hpjosu3nzW3L+fK68//5y3TjQS88YjzFeq7sW492eeR8wbWdIkDGdzNsNdVuQU1pqoiERO2hIxBWsDCm13UDLZ0hjn31MmUTzdi1k4aYl+nz0vvv99HHSuGUTWbJtpc86mramLfZ5hoQrfbz/o1Wc6EiAh+p33NFRTaPixLbwbAPPQo4e3SxdurSV558fo8cxdGgfGTVqgFoX0/n5G2Tjxjf08MGDe8gDD/RSFS2eBe3atUzNN1bKVhW03XqIH/v04YdJ6vkL9k0zDjyrQdi3365X0+Z48UwG+2t8HmZ8rmNcFtNavE7pZN4uFMq2NDmlJURDInbQkIgrWBkSRu226mWXnJcmt/TqqipAdPWGMbW/9Sap3aCOmle91jWyIjvRbz1o57TpPr3s0GEAD97r1Kkhl15aSTUP4cH9v/7lW6Ebe9mNHj3Qp5cdKtxx44aobtFokmrevKEsWDBJD0fl+vDDd6u7CfQcGznSt5ed+b/TetCUKSNVD7vGjevKrFlP+dzJjB//oFoHTV6rV8/1iRfGgOPDcVx9dTV54omHVIcFhJn3AdNavE7pBBm3u3fvOyFtS5NTWkI0JGIHDYm4gpUh4RMSHov3kCA8Pxr/5yfV+0eVKldSzUPX1K4hvYf0kUVZSy3XyfhkraxvN8jnPSQn2VWcVNmJhkTsoCERV7AyJIyogE9IBDu6t5N2L/o/efcm/5EaAomGFHmiIRE7aEjEFSwNqUj4nhE+IXGhY9ll5abJmmb3hDyWHQ0p8kRDInbQkIgr2BkShO8Z4RMSJR3te93Ha2Vz75Gy90mO9h0LoiERO2hIxBUCGRK+Y4TvGeETEqF+Dylrb7oyo+338ntIsSIaErGDhkRcIZAhQTATfM8In5DAqN1W5mMUOjDgmRGa6XBnRDOKHdGQiB00JOIKToakCc+AMtsPloy2AyVz4lRZ53lL0jYlSPrHa2X91kTZnr5Ydkx9XtbfMEjevfH+iPr+EeWOaEjEDhoScYVgDUnpfK7quo33iTZ0HCIZ9XqrdfGLacxXXbtD6E1HRY9oSMQOGhJxhZAMiYpr0ZCIHTQk4go0JCpY0ZCIHTQk4go0JCpY0ZCIHTQk4gqaIX0+f46qcCjKTvvnzaEhEUtoSMQVNEOiqGBFQyJmaEjEFfQ7pAWzVbMdRdnp8/mzaUjEEhoScQU+Q6KCFZrtaEjEChoScQUaEhWsaEjEDhoScQUaEhWsaEjEDhoScQUaEhWsaEjEDhoScQUaEhWsaEjEDhoScQXNkPgeEuUkvodE7KAhEVfQDImighUNiZihIRFX0O+Q+B4S5SC+h0TsoCERV+AzJCpYodmOhkSsoCERV6AhUcGKhkTsoCERV6AhUcGKhkTsoCERV6AhUcGKhkTsoCERV3AypBEj+knz5g1l8OAecvjwJunWrb2a/+2366Vatcv8/tsJ633zzTrLZTGvevVqPvNCUTDbD6fWrfurjBkzyDIslkRDInbQkIgrBDKkgoItUqVKZTl3bo9fWCiGlJ6+QIYP76v+x6IhQe3bt5B//nOtZVisiIZE7KAhEVewM6TCwvekdesmUr58ebnhhuby0ktP2pqQkyEMGHCHZGS8rP5bLYt5RkOaM2eidOx4vVx/fVPp0KG1fPxxsh72l79MkOuuayBt2zZT+2fcr+efH6OWb9Sojmzc+IYeN8KmTx+r4mzSpK4Kmzr1UTWNZbOy3nTcthbPhAkPS7t216llDh7M1Nf7858flxkzfqdPx6JoSMQOGhJxhUB3SGbzME7b/Tfr/PlcufLKy9XdFqaxrGZymmAsRkP6/vut+v+UlDnSo0cn9f+HH7ZJxYoVfMLxH3GWK1dO3Ylh3jvv/K8yDPzXwlavnqumk5Nnq7s+zSCTkl6STp3aqP+Q3ba1eLA+pp97brQMHdpHX3bTpjele/cb9elYFA2J2EFDIq4QbkP67rstUqnSJfq01bKYZzSk1NR50rlzG3WXArOqVau6mg9zQ6V/1123yKxZT8nOnUv09StX/nUbBw78XWrUuMonDOtqYVWrXmq5LGS3bcQDM/zPf4qbLz/5JEUaNLhWX2///lSpW7emPh2LoiERO2hIxBUizZB+/HGXuoPJzU1Q0//4xyofs4Kx7NmTIK+9Nk3uvLOjjBzZ3y9OY3yBwszTgbaN5QIZ0uefp0mdOjX06VgUDYnYQUMirlAaTXZXXFHVp8nOvCzmaRX/sWPb5eKLK8qhQxvV9MSJw/WwEyd2qGW19Xbs+Jt6JmSO0xhfoDDzdKBtYzk02a1aVdz0h2dSw4YVd9SA8Byqa9fiHoixKhoSsYOGRFwh3IYE9e//X7JmTfCdGv70p8fU3Qaazl544fd6WH7+Br05DR0Lbr65laxf/6pfnMb4AoVZTdttW4vHrlPDf//3OGVS2nQsioZE7KAhEVcIZEhuydjtO1plNjazbryxpXz99RrLsFgRDYnYQUMirlAahoRmO+3FWKvwaFAgQ8KLsaNHD7QMiyXRkIgdNCTiCqVhSFRsiIZE7KAhEVegIVHBioZE7KAhEVegIVHBioZE7KAhEVcIlyEdP/6ezJz5hN987VmScaBWs4w93+Jl4NJoEA2J2EFDIq4QLkOy6gQQbG87oyFB8TBwaTSIhkTsoCERV3AyJAzPg8FM8d7NlCkjfYzi6NHN8tBDvdV7ORhmp2/fbvq7OXj3SBuzThsrLtAgq7t3L1OfucC7RZMmjfDZTjwMXBoNoiERO2hIxBWcDKlly0b66AR4adRoFL16dZG1a1/Rp5cv/x9lSvhvNhx0/TYPsmoMh6F5PLPUfwxcatxOPAxcGg2iIRE7aEjEFQIZEsahM47fZhzbDc+IKlTwHbUbn4SAEG42nEBj2pm3g3HijIYUDwOXRoNoSMQOGhJxhZIaEj4FgXHffvppt886mkIxJHzyIZAhxcPApdEgGhKxg4ZEXMGpya5Fi0bqkwz4j/HajEbRs2dnNX6b9mmHM2fel7y84ng0Azp79gM17TTIKprstKZBPC8ybiceBi6NBtGQiB00JOIKTob03nuLpVmz+urrqU8+OdSn6Qzm8pvf3CutWjVWHRvwi+8UaeEjRvSTpk3r6Z0aAg2yumvXMtV5AtuZPNm3U0M8DFwaDaIhETtoSMQVnAzp1Kls/f/s2X+Qfv1u8wkPRSUdZDUeBi6NBtGQiB00JOIKToaEF1PRYQFdstH1Gx0MrJYLRiUZZDVeBi6NBtGQiB00JOIKToZEUZpoSMQOGhJxBRoSFaxoSMQOGhJxBRoSFaxoSMQOGhJxBc2Q9s+boyocirIT8ggNiVhBQyKuoBkSRQUrGhIxQ0MirnD2+Ek5ujVXV+a8heJ5Ya4snzpTlk15UVU+paI//o8kdRkuiZW7StLkmdbLxJuK0kGlR1G6qPSxWqYs9LdlNCTiAw2JhIXs7GzJyMiQhIQEVemEW0tfeV1W9h4jiVW7yzvX9JCV3X9juVy8CumBdEH6IJ2QXlbLlYWQR5BXkGcOHDjgzUEkHqEhkbCQl5enrnhTU1NVhYOr4LDIYET4Xfby66riXf6nOdbLx6mQHkgXpI8xvZB+VsuXlpA3kEeQV5Bn8vPzvTmIxCM0JBIW0PSSk5OjKhpc/aJJxlUtXCJJ9zymKlb8Yhrzkyb/RRKbD/BfnlLpgvRR0zbpV9pC3kAeQV5BnikoKPDmIBKP0JBIWMCVLioYXPWiKQaVjhvamJIuGfdPFM/l/6V+MW0MX91hmKybNtdnHlUspAvSxzjPKT3DLeQN5BHkFeSZwsJCbw4i8QgNiYQFVCy42kUlg+cCqHAuRP/I/kC2jJwhKdXuUL+YNi/z8bvbZFWNu+Tzf3zqF0Z9odIF6YN0MocFk77hEPIG8gjyCvLMmTNnvDmIxCM0JBLR/HT4e8n7w3xZfVUP9YtpO/ZOmCv7nnvDO0WsQPognewIJb0JcRsaEolIQq0Yz508Laur95IfDx7xziFWIH2QTkivQNCYSFlAQyIRRUkrwq9eXyU7Bj/jnSKBQDohvYKBxkRKExoSiRjOHP1BXb1v7T0h5IpvfZuH1WgRxBmkE9IrFHA+cF5wfnCeCAkHNCQSUWCUh7SavdVvsJSkgo13QjXwkpwXQkKFhkQijlArv1CaoEgxoTRx0oxIaUFDIhFJsJVgsA/piS/BdgKhGZHShIZEIhatMjycucs7xx+nbszEHqdu8kh3mhEpTWhIJKL56tVkSarQxdKUzv98VtKu7SMn9n/jnUNCAemG9EM6mkF6I92R/oSUFjQkErEc++hLSat1t+yfvcLySv2bFetl611PeKdISUD6IR2NaHemKt2L0h/ngZDSgIZEIpKTXx6U9Lr3yLfJWWpaqySNprTxllHy7/Tt3ilSEpB+SEcNczoj/XEecD4ICTc0JBJxnP73UVnTeID8c1GGd04xxsryhz2fyZpG98kv5897Q0lJQPohHZGeZjPSwHnA+cB5ISSc0JBIRHGm4JisazVE9s9Z4Z3ji1Zpbuo6Rj57aZl3LrkQkI5ITysz0sD5wHnB+SEkXNCQSMRw7sSPsqHDCNn37OveOdag0kSX5Z+/O+6dQy4EpCPS086MNHBecH5wnggJBzQkEhH856efZfPtv5c9j8/yzgnM6Xx+yM1Ngk1PnB+cJ5wvQtyGhkTKnF/O/Ue293tadg2bzmdCEQ7OD84TzhfOGyFuQkMiZYqq4IbPkO33TmQFFyWoC4ii84XzxgsI4iY0JFKm5I5jE1A0ojWx4vwR4hY0JFJmYNgaPCQ/W3jKO4dEEzhvqhMKv9JLXIKGRMqE/XNXshtxDKB30y86n4RcKDQkUuqoFy0b3cfPjccIOI84n+YXmQkJFRoSKVUOpmyW9Dp9ORRNjKGGeio6rzi/hJQUGhIpNQ69u7t4sM4Pv/DOIbEEzivOL84zISWBhkRKhe+y90lqjd5SsPMj7xwSi+D84jzjfBMSKjQkEna0z0jwyjk+0O+E+dkKEiI0JBJWzJ+RIPEBP1tBSgINiYQN7TMSX7+d7p1D4gmcd362goQCDYmEBafPSJD4gJ+tIKFAQyKuE+xnJEh8wM9WkGChIRFXCfUzEiQ+4GcrSDDQkIhr8DMSxA5+toIEAw2JuIJe4fAzEsQG/bMVvGAhNtCQiCuwSYYEA5t0SSBoSOSC4UNrEgrs9ELsoCGRC4LdeklJ4GsBxAoaEikx6jMSjQfwMxKkRKjPVhTlH362gmjQkEiJ4NAwxA04tBQxQkMiIcPBM4mbcPBdokFDIiGhfUaCnxcgbsJ8RQANiQQNr2RJOOGdN6EhkaDgJ6pJacBP3Mc3NCTiCHtDkdKEvTfjFxoSCYj+vsjcld45hIQf5De+3xZ/0JCILWcLTxW/Uf/cG945hJQeyHfIf8iHJD6gIRFLtDHHcsdxzDFSdiD/cYzE+IGGRPzQR2UePoOjMpMyRY0iX5QPOYp8fEBDIj7wuzUk0lAXSPzOVlxAQyI+8DMSJBLhZyviAxoS0eFnJEgkw89WxD40JKLgZyRINMDPVsQ2NCQiX7+drl5EPP3vo945hEQuyKfIr8i3JLagIcU5/IwEiUb42YrYhIYUx3AwSxLNcLDf2MPHkM4eOyFHtuRScaBDmbsk7do+8uVfky3DqeB09vhJb+kpO+K53CL/Ih8jP1uFU5Epu3LjY0hYMLFcJ4qigtTRrbne0lN2sNxS0Sa7cmNpSPvnzyn6n0hRlI32z5sTsGCVJiy3VLTIqdxYGhJW/OWXvRRF2ejw5sSIMySWWyrS5VRuaEgUVQJpBStz3kLJzs6WvLw8+eKLLyQ/P18KCwu9Jap0YLmlokVO5YaGRFElkFawPC/MlYyMDMnKypKcnBxVuAoKCrwlqnRguaWiRU7lpsSG9PPZD2TfmqWybvRESW47UJJq9Sze0LU9JfWGQbLt8UmSv/kd+eV8ruX6FBXN0grW8qkzJSEhQVJTU1XhwhUfrvZKE5ZbKlrkVG5CNqRz5/fKB56F4ml1nyS16C+ecZMkZeXrkrJhhaz6KEP9YhrzPS37S3pRps/fsNIyLoqKVmkFa9mUF2XJkiWqcOGKD80QBw4c8Jao0oHllooWOZWbkAzp1OkcyRgxQRIb9JbkebNURnYSlktueLfkjJ8s58/xqouKDWkFK2nGHFWwli5dKh6PR13tofmhNGG5paJFTuUmaENCpk65fbh4ug2VVdnJlpnYVkXLp3QfJlv6jmLmpmJC0WJILLdUJMkVQ8LtPq6wVKbOS7fOvE4qWm/1bcNlz4QpPnFTVDQqGgyJ5ZaKNLliSGh7xu3+qp0hXmGZVbT+6sZ95BDbpstU+/enSrVql+nTM2c+IVdeeblUrXqp5OUl+oS5IfP2YkHRYEhlWW5DOeeffrpaKlQobxkGIbxSpUssw0pTLDcXrgs2JPTK8bTub9n2nFJ09fTYjPHSpFVTqVCxglx8ycXStHUzadOxrbS++Xq/5aHUBbMl88bBPr14kOHKlSunTuzll1eVevVqydChfeSTT1L0ZaJdVoUuJ2eFdOvWXqpUqayOu1276yQtbb7PMuHW8ePvqf367LPVluElkVMFcyEKlGbh3K5ZkW5IpV1uNXXseL0eHqyczhvCjYbEchO6oqXcOBoSuogmtejnl0FTPkyX2/vdqTIkMnSrm66Xlu1bqQyOeVUuq+K3jqaMNgPkyNYkfRvmBPnuuy0ya9ZT6urg44+T9fnRLPMxnjnzvtSsebW88soz8vPPOXLu3B7ZvXuZbNu2yGe9cAtXYW5nxnBlcKc0u9Dtni2qxK3mWynSDaksym1J5RQPwjVDYrkJXdFUbhwNad3oP0jyuEl+mfPp2c+oDNzwuoby1obF+vw3MxfJ1TWrB8zYmU9Pk7yJ0/Rt2CXIxInD5aGHeuvTBQVb5P77e8oVV1SV6tWryVNPDfVJjB9+2CbDhvVVt9Ews9/+tr+ab47fmMG1sNdem6ZOWoMG18qOHX+T+fP/qKbr1q2pprV1T57cKWPGDFLbx3YeffQ+OX16t19cNWpcpe70tm9frMJq175GpZd2JZmRsUBN//RT8bpmaXG9/fYMqV+/lrqqGTt2kMpc2jKB9gVySo+PPvKoKyZtv3r16qKHOcUBvfrqVJVeiKNz5zZFGSpdzTcf66pVc3ziDHQeA6XhV1+tCZhm5u3m5iYEta2FC6dLw4a1pWfPzmq+U7pCkW5IZVlurcKCKTfaskeOZEnfvt3UOWzRopHKZ1p5dcoDWlwsN9FZbhwNKaXoqihlxRt+mfPm7h3UQby49CW/sFFTxkrvB/r4zde0Lult2dDhAX0b5pOpacuWhSpTadP9+t0mDz54l5w6lS2HD2+SDh1ay4wZv9PDBw26U+nYse0q8bKzl6r55vgxbTQkHMe0aaPUlQPigwk999xofbpr1/b6uo88co8MGdJLCgvfkxMndsi993aXyZNH+MT17LPFceH3llva6WHGfcCVCjIN9jc9fYE6Hi1MWx5xIVPjeFFIO3VqI9Onj9WXCbQvUDDpYd4v87RdHNCGDa+rjItjwb6gErGKwzwd6DxiWbs0DCbNjNuBgtnW448PUYVVKzxO6QpFuiGVZbm1CnMqN+b8gSb7H3/cpfL9rbfeoJdXlpvYLjeOhpRU805J2ZTglznrNamvdsqTm+oX5qS0rERJr9NL34ZVgkD79nnksqIrNvxHm+1FF10k33yzTg/PyHhZmjWrr4eXL19eDhz4ux6uyRw/ps2G9P33W9X0hx8mqWnEp03jKgH/cQVQsWIFOXgwU01DO3cukUaN6qj/5rhwpYErNC3MfIxff71GXb01bVpPHRvaeLWrJS0u4/GuWfPr8TrtS7DpYd4v43SgOMxCEwCujPDfKc5A5xHL2qUh5JRmxu0Guy1UWlq4U7pqinRDKq1yi7hwB6AJzUJamHYugik35vzxr3+t15dFvtfKK8RyUxwei+XG0ZASL7lVVu3xz7x1GtVVO4UHpOYwJ6XvTRdPpa76NswJomnz5rf0OyQkHhLJGG40LKtwTeb4MW00JLsw8/SXX2b4FUCY1VVXXaEvG+x2zDp0aKNqnjTeUQU6Xqd9CTY9rPZZmw4UB/Tmm8+pTIqMD1WubJ+mgeI0HpfVusbzYZRVmhnXDXVbkFO6aop0QyrLcmsOC6Xc2J2zUPJAoHPOchPZ5cbRkDw1e1heaeFhKHbg5dRX/cKclLndE9Qd0tNPP6I/Q7JybeOVD24TcVViDNektaFq7ch4mGdnFOYTaZzGPmBZ/GrhRgWKC71xrI7RKFzV4CTiP9bFPhuPZ+3aV9QVDv477Uug9DDup9U+a9OB4kCTA5bDRcP587ny/vvLbY/VGKfTebTaH+P5MMuYZubthrotyCldNUW6IZVluTWHOaWpeVmcM+MdEvJ9sHkAcbHcFE9HY7lxNKTUdgMt26LHPvuYOvHN27WQhZuW6PPRi+f308dJ45ZNZMm2lT7raNqatjhgWzRuO+fOfVolGB4gavPRJom2ZWPbsNauCQ0e3EMeeKCXShS02+7atUzNRxsqrkQSEv6i2jzRNqqdLKcTaZ7G9keNGqAeWmI6P3+DbNz4hr6sXVxoh0V6ffttcUHDVQreY9BOOuLDA8E77uior4vl0S6L4z16dLN06dJWnn9+jAqHAu0LZJcexv202mfjtF0c2Bb2D02aSFPsp92xmuMMdB6t9keL1ynNzNuFQtmWJqd0hSLdkMqi3BplDgul3OCcDR/eV3+GhOalYPMA4mK5KZ6OxnLjaEgY/deqt05yXprc0qurOhB0GUUGb3/rTVK7QR01r3qta2RFdqLfetDOadP9eutgHfTwgHGgU8HDD9+tbhO1ZSAkDB7MYZmrr64mTzzxkDIbLRwJgfVgZOjlMXLkrz1b0OsG89ArBD3otJMV6ERaTeMEjRs3RPXAwy1s8+YNZcGCSfqygeIaP/5BtW+4nd2+fZEyxjp1asill1ZSt7Y4Nu3KUIvL2Fto9OiBPr2FAu0LZJcexv202mfjdKA0nTJlpOop1LhxXdVN3+5YV6+e6xNnoPNotT9avHhAHCjNION29+59J6RtaXJKVyjSDam0yq1V+lmFhVJuUEHeffetKm+1bOnby84pD2hxsdxEZ7lxNCQMRe+xeJ8BQjv0+D8/qd5jqFS5krrNu6Z2Dek9pI8sylpquU7GJ2tlfbtBPu8zUP6yO+lUZCjSDSleyy3LTWTrgg0Jb2ZjKPpgRwl20u5F/yfv3uT7xjflLxasyFakG1K8lluWm8jWhRtSkfBdFAxFf6FjYmXlpsmaZvdwLLsgxIIV2Yp4QypSPJZblpvIliuGBOG7KBiKvqSjBq/7eK1s7j1S9j7JUYOp6Fc0GBLEcktFklwzJHwPBd9FwVD0oX5XJWtvusrU2+/ld1Wo2FC0GBLLLRVJcs2QIGRKfBcFQ9GnLphtmYmNwoNQtD3jdh9XWMzUVKwoWgwJYrmlIkWuGpImtCVnth8sGW0HSubEqbLO85akbUqQ9KLb+/VbE2V7+mLZMfV5WX/DIHn3xvv5zIiKOUWTIWliuaXKWmExJKXzuaoLKN5L2NBxiGTU663WxS+mMV91EWVvOioGFY2GpMRyS5WhwmdIFBXHilpDoqgyFA2JosIgGhJFhS4aEkWFQTQkigpdJTKkz+fPUStSFGWt/fPmRJwhsdxSkS6ncmNpSBRFBadIMiSKihaFdoe0YHbR/0SKomz0+fzZAQtWacJyS0WLnMoNnyFRVAmE5odIMySWWyrS5VRuaEgUVQLRkCgqdNGQKCoMoiFRVOiiIVFUGERDoqjQRUOiqDCIhkRRoatEhsT3GSgqsPgeEkWFLr6HRFFhVCQZEkVFi0K7Q+L7DBQVUHwPiaJCF99DoqgwCM0PkWZILLdUpMup3NCQKKoEoiFRVOiiIVFUGERDoqjQRUOiqDCIhkRRoYuGRFFhEA2JokJXiQwJfcWxIkVR1orE95BYbqlIF99DoqgwKpIMiaKiRUEZ0tnjJ+Xo1lxdmfMWiueFubJ86kxZNuVFFQlFUQb9bVmZGxLLLRV1sik3PoZkJjs7WzIyMiQhIUGtTFGUtVBGUFZQZg4cOOAtQWUDyy0VLTKXm4CGlJeXp5wrNTVVrQg3oyjKVygbKCMoKygz+fn53hJUNrDcUtEgq3IT0JBwC5WTk6NWgIvh1oqiKF+hbKCMoKygzBQUFHhLUNnAcktFg6zKTUBDgmNhQbgXbqmwMkVRvkLZQBlBWUGZKSws9JagsoHllooGWZWbgIaEBeBaWBjte1iRoihfoWygjKCsoMycOXPGW4LKBpZbKhrkX27OyP8DXb/ok+It9KcAAAAASUVORK5CYII="}}]);