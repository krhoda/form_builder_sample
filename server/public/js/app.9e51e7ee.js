(function(e){function t(t){for(var a,i,l=t[0],c=t[1],s=t[2],d=0,v=[];d<l.length;d++)i=l[d],Object.prototype.hasOwnProperty.call(n,i)&&n[i]&&v.push(n[i][0]),n[i]=0;for(a in c)Object.prototype.hasOwnProperty.call(c,a)&&(e[a]=c[a]);u&&u(t);while(v.length)v.shift()();return o.push.apply(o,s||[]),r()}function r(){for(var e,t=0;t<o.length;t++){for(var r=o[t],a=!0,l=1;l<r.length;l++){var c=r[l];0!==n[c]&&(a=!1)}a&&(o.splice(t--,1),e=i(i.s=r[0]))}return e}var a={},n={app:0},o=[];function i(t){if(a[t])return a[t].exports;var r=a[t]={i:t,l:!1,exports:{}};return e[t].call(r.exports,r,r.exports,i),r.l=!0,r.exports}i.m=e,i.c=a,i.d=function(e,t,r){i.o(e,t)||Object.defineProperty(e,t,{enumerable:!0,get:r})},i.r=function(e){"undefined"!==typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},i.t=function(e,t){if(1&t&&(e=i(e)),8&t)return e;if(4&t&&"object"===typeof e&&e&&e.__esModule)return e;var r=Object.create(null);if(i.r(r),Object.defineProperty(r,"default",{enumerable:!0,value:e}),2&t&&"string"!=typeof e)for(var a in e)i.d(r,a,function(t){return e[t]}.bind(null,a));return r},i.n=function(e){var t=e&&e.__esModule?function(){return e["default"]}:function(){return e};return i.d(t,"a",t),t},i.o=function(e,t){return Object.prototype.hasOwnProperty.call(e,t)},i.p="/";var l=window["webpackJsonp"]=window["webpackJsonp"]||[],c=l.push.bind(l);l.push=t,l=l.slice();for(var s=0;s<l.length;s++)t(l[s]);var u=c;o.push([0,"chunk-vendors"]),r()})({0:function(e,t,r){e.exports=r("56d7")},"56d7":function(e,t,r){"use strict";r.r(t);r("e260"),r("e6cf"),r("cca6"),r("a79d");var a=r("2b0e"),n=function(){var e=this,t=e.$createElement,r=e._self._c||t;return r("v-app",[r("v-main",[e.targetForm?r("div",[r("p",[e._v("Will Be A Form")])]):r("div",[r("Container")],1)])],1)},o=[],i=(r("d3b7"),r("ac1f"),r("3ca3"),r("841c"),r("ddb0"),r("2b3d"),function(){var e=this,t=e.$createElement,r=e._self._c||t;return r("v-container",[r("v-row",{staticClass:"text-center"},[r("v-col",{attrs:{cols:"12"}},[r("v-row",{attrs:{align:"center",justify:"space-around"}},[r("v-btn",{attrs:{elevation:"2"},on:{click:function(t){return e.updateNav("builder")}}},[e._v(" Form Builder ")]),r("v-btn",{attrs:{elevation:"2"},on:{click:function(t){return e.updateNav("file-search")}}},[e._v(" Form Finder ")]),r("v-btn",{attrs:{elevation:"2"},on:{click:function(t){return e.updateNav("submission-search")}}},[e._v(" Submission Search ")])],1)],1),r("v-col",{staticClass:"mb-4"},["none"===e.currentNav?r("div",[r("p",[e._v("Select a page")])]):e._e(),"builder"===e.currentNav?r("div",[r("FormBuilder")],1):e._e(),"file-search"===e.currentNav?r("div",[r("p",[e._v("Will be a form finder")])]):e._e(),"submission-search"===e.currentNav?r("div",[r("p",[e._v("Will be a submission search")])]):e._e()])],1)],1)}),l=[],c=function(){var e=this,t=e.$createElement,r=e._self._c||t;return r("div",[""!==e.shareID?r("div",[r("v-card",{attrs:{elevation:"2"}},[r("v-card-title",[e._v("The form ID is: "+e._s(e.shareID))]),r("v-card-text",[e._v(" To view it now: "),r("a",{attrs:{target:"_blank"}},[e._v("click here")])]),r("v-card-actions",[r("v-btn",{attrs:{color:"primary"},on:{click:e.backToForm}},[e._v("Make Another Form")])],1)],1)],1):e._e(),""===e.shareID?r("div",[r("v-card",{attrs:{elevation:"2"}},[r("v-card-title",[e._v(" Form Builder ")]),r("v-card-text",[r("p",[e._v("Set the label, set the type, add it to the form!")]),r("v-text-field",{attrs:{label:"Field's Label"},model:{value:e.label,callback:function(t){e.label=t},expression:"label"}}),r("v-radio-group",{scopedSlots:e._u([{key:"label",fn:function(){return[r("div",[e._v("Set the type")])]},proxy:!0}],null,!1,1247088660),model:{value:e.input_type,callback:function(t){e.input_type=t},expression:"input_type"}},[r("v-radio",{attrs:{value:"text"},scopedSlots:e._u([{key:"label",fn:function(){return[r("div",[e._v(" String ")])]},proxy:!0}],null,!1,2357185346)}),r("v-radio",{attrs:{value:"number"},scopedSlots:e._u([{key:"label",fn:function(){return[r("div",[e._v(" Number ")])]},proxy:!0}],null,!1,2666811636)})],1)],1),r("v-card-actions",[r("v-btn",{attrs:{color:"primary"},on:{click:e.addToSchema}},[e._v("+ Add to Form")])],1)],1),r("v-card",{staticClass:"mt-10",attrs:{elevation:"2"}},[r("v-card-title",[e._v("The form will look like:")]),r("v-card-text",[0===Object.keys(e.schema).length?r("div",[e._v(" None. ")]):e._e(),e._l(e.schema,(function(t){return r("div",{key:"field-"+t.label},["text"===t.input_type?r("v-text-field",{attrs:{"prepend-icon":"mdi-format-text",label:t.label}}):r("v-text-field",{attrs:{"prepend-icon":"mdi-numeric",label:t.label,type:"number"}}),r("v-btn",{attrs:{color:"error","x-small":""},on:{click:function(r){return e.deleteFromSchema(t.label)}}},[e._v("Remove "+e._s(t.label))])],1)})),e._v(" "+e._s(JSON.stringify(e.schema))+" ")],2),r("v-card-actions",[r("v-btn",{attrs:{color:"success"},on:{click:e.createForm}},[e._v(" Save Form & Generate Link ")])],1)],1)],1):e._e()])},s=[],u={name:"FormBuilder",data:function(){return{schema:{},label:"",input_type:"",shareID:""}},methods:{addToSchema:function(){var e=""!==this.input_type&&""!==this.label;if(e){var t=this.schema[this.label];t?alert("Field ".concat(this.label," has already been added!")):(this.schema[this.label]={input_type:this.input_type,label:this.label},this.input_type="",this.label="")}else alert("Label and type are required!")},deleteFromSchema:function(e){var t=Object.assign({},this.schema);delete t[e],this.schema=t},createForm:function(){var e=this;fetch("http://localhost:8000/create-form",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({schema:this.schema})}).then((function(e){return e.ok&&200===e.status?e.json():(alert("Bad result in Form Submit, non-200 status code: ".concat(e.status)),!1)})).then((function(t){t&&(e.shareID=t.form_id)}))},backToForm:function(){this.schema={},this.shareID="",this.input_type="",this.label=""}}},d=u,v=r("2877"),p=r("6544"),f=r.n(p),h=r("8336"),b=r("b0af"),m=r("99d9"),_=r("67b6"),y=r("43a6"),g=r("8654"),x=Object(v["a"])(d,c,s,!1,null,"5e8e4390",null),k=x.exports;f()(x,{VBtn:h["a"],VCard:b["a"],VCardActions:m["a"],VCardText:m["b"],VCardTitle:m["c"],VRadio:_["a"],VRadioGroup:y["a"],VTextField:g["a"]});var F={name:"Container",components:{FormBuilder:k},data:function(){return{currentNav:"none"}},methods:{updateNav:function(e){this.currentNav=e}}},S=F,w=r("62ad"),O=r("a523"),j=r("0fd9"),T=Object(v["a"])(S,i,l,!1,null,null,null),N=T.exports;f()(T,{VBtn:h["a"],VCol:w["a"],VContainer:O["a"],VRow:j["a"]});var V={name:"App",components:{Container:N},created:function(){console.log("Query is: ".concat(window.location.search));var e=new URLSearchParams(window.location.search),t=e.get("form");console.log("Target is: ".concat(t)),t&&(this.targetForm=t)},data:function(){return{targetForm:""}}},C=V,B=r("7496"),P=r("f6c4"),D=Object(v["a"])(C,n,o,!1,null,null,null),I=D.exports;f()(D,{VApp:B["a"],VMain:P["a"]});var A=r("f309");a["a"].use(A["a"]);var M=new A["a"]({});a["a"].config.productionTip=!1,new a["a"]({vuetify:M,render:function(e){return e(I)}}).$mount("#app")}});
//# sourceMappingURL=app.9e51e7ee.js.map