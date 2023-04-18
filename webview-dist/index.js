var t=Object.defineProperty;function e(t,e=!1){let n=window.crypto.getRandomValues(new Uint32Array(1))[0],o=`_${n}`;return Object.defineProperty(window,o,{value:n=>(e&&Reflect.deleteProperty(window,o),null==t?void 0:t(n)),writable:!1,configurable:!0}),n}async function n(t,n={}){return new Promise(((o,i)=>{let r=e((t=>{o(t),Reflect.deleteProperty(window,`_${s}`)}),!0),s=e((t=>{i(t),Reflect.deleteProperty(window,`_${r}`)}),!0);window.__TAURI_IPC__({cmd:t,callback:r,error:s,...n})}))}function o(t,e="asset"){let n=encodeURIComponent(t);return navigator.userAgent.includes("Windows")?`https://${e}.localhost/${n}`:`${e}://localhost/${n}`}((e,n)=>{for(var o in n)t(e,o,{get:n[o],enumerable:!0})})({},{convertFileSrc:()=>o,invoke:()=>n,transformCallback:()=>e});var i={};!function(t){Object.defineProperty(t,"__esModule",{value:!0}),t.TableGenerator=t.CLASSIC_COLUMN_ID=void 0,t.CLASSIC_COLUMN_ID={name:"id",type:"INTEGER",options:{primaryKey:!0,autoIncrement:!0}};class e{constructor(t,e){this.name=t,this.options=e,this.columns=[]}static fromData(t){const n=new e(t.name);return n.options=t.options,n.columns=t.columns,n.foreignKeys=t.foreignKeys,n}setTableOptions(t){this.options=t}addClassicIdColumn(){return this.addColumn(t.CLASSIC_COLUMN_ID.name,t.CLASSIC_COLUMN_ID.type,t.CLASSIC_COLUMN_ID.options)}addColumn(t,e,n){return void 0===this.columns.find((e=>e.name===t))&&(this.columns.push({name:t,type:e,options:n}),!0)}addForeignKey(t){void 0===this.foreignKeys&&(this.foreignKeys=[]),this.foreignKeys.push(t)}getCreateTableString(){if(void 0===this.options)return`CREATE TABLE ${this.name}`;let t=["CREATE"];return!0===this.options.temporary&&t.push("TEMP"),t.push("TABLE"),!0===this.options.ifNotExists&&t.push("IF NOT EXISTS"),t.push(this.name),t.join(" ")}getCreateColumnString(t){const e=[t.name,t.type];return void 0!==t.options&&(!0===t.options.primaryKey&&e.push("PRIMARY KEY"),!0===t.options.autoIncrement&&e.push("AUTOINCREMENT"),!0===t.options.notNull&&e.push("NOT NULL"),!0===t.options.unique&&e.push("UNIQUE"),void 0!==t.options.onConflict&&e.push(`ON CONFLICT ${t.options.onConflict}`),void 0!==t.options.default&&("string"==typeof t.options.default?e.push(`DEFAULT "${t.options.default}"`):e.push(`DEFAULT ${t.options.default}`))),e.join(" ")}getCreateForeignKeyString(t){let e=`FOREIGN KEY(${t.column}) REFERENCES ${t.targetTable}(${t.targetColumn})`;return void 0!==t.onUpdate&&(e+=` ON UPDATE ${t.onUpdate}`),void 0!==t.onDelete&&(e+=` ON DELETE ${t.onDelete}`),e}getCreateTableQuery(t){void 0===t&&(t=!1);const e=this.getCreateTableString(),n=this.columns.map((t=>this.getCreateColumnString(t)));let o=[];return void 0!==this.foreignKeys&&(o=this.foreignKeys.map((t=>this.getCreateForeignKeyString(t)))),t?`${e} (\n${n.join(",\n")}${o.length>0?",\n"+o.join(",\n"):""}\n);`:`${e} (${n.join(",")}${o.length>0?","+o.join(","):""})`}}t.TableGenerator=e}(i);var r=function(){function t(t){this.path="",this.path=t}return t.prototype.connect=function(t){return void 0===t&&(t={}),n("plugin:sqlite-store|open",{dbPath:this.path,options:t})},t.load=function(e){return new Promise((function(o,i){var r=new t("");void 0===e&&(e={}),n("plugin:sqlite-store|load",{options:e}).then((function(t){r.path=t,o(r)})).catch(i)}))},t.open=function(e,n){return new Promise((function(o,i){var r=new t(e);r.connect(n).then((function(t){t?o(r):i("Unable to open database ".concat(e,"."))})).catch(i)}))},t.prototype.getTableGenerator=function(t){return new i.TableGenerator(t)},t.prototype.create=function(t){var e=i.TableGenerator.fromData(t).getCreateTableQuery();return this.execute(e,[])},t.prototype.setPragma=function(t,e){return n("plugin:sqlite-store|set_pragma",{dbPath:this.path,key:t,value:e})},t.prototype.select=function(t,e){return void 0===e&&(e=[]),n("plugin:sqlite-store|select",{dbPath:this.path,query:t,params:e})},t.prototype.selectFirst=function(t,e){var n=this;return new Promise((function(o,i){n.select(t,e).then((function(t){t.length>0?o(t[0]):i(new Error("No results"))})).catch(i)}))},t.prototype.execute=function(t,e){return void 0===e&&(e=[]),n("plugin:sqlite-store|execute",{dbPath:this.path,query:t,params:e})},t.prototype.batch=function(t){return n("plugin:sqlite-store|batch",{dbPath:this.path,queries:t})},t.prototype.close=function(){return n("plugin:sqlite-store|close",{dbPath:this.path})},t}();export{r as SqliteStore};
