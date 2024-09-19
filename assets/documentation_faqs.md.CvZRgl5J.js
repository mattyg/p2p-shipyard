import{_ as e,c as o,o as t,a1 as a}from"./chunks/framework.rLYnVynv.js";const b=JSON.parse('{"title":"Frequently Asked Questions","description":"","frontmatter":{},"headers":[],"relativePath":"documentation/faqs.md","filePath":"documentation/faqs.md"}'),s={name:"documentation/faqs.md"},i=a('<h1 id="frequently-asked-questions" tabindex="-1">Frequently Asked Questions <a class="header-anchor" href="#frequently-asked-questions" aria-label="Permalink to &quot;Frequently Asked Questions&quot;">​</a></h1><h2 id="does-this-mean-that-holochain-already-supports-mobile" tabindex="-1">Does this mean that holochain already supports mobile? <a class="header-anchor" href="#does-this-mean-that-holochain-already-supports-mobile" aria-label="Permalink to &quot;Does this mean that holochain already supports mobile?&quot;">​</a></h2><p>Well, not quite. Let&#39;s break it down to the two main mobile platforms:</p><h3 id="android" tabindex="-1">Android <a class="header-anchor" href="#android" aria-label="Permalink to &quot;Android&quot;">​</a></h3><p>Holochain has experimental support for Android. This means that holochain works as expected on Android, <strong>except for these issues</strong>:</p><ul><li><a href="https://github.com/holochain/holochain/issues/3901" target="_blank" rel="noreferrer"><code>delete_link</code> bug in holochain core</a>.</li><li><a href="https://github.com/holochain/holochain/issues/3243" target="_blank" rel="noreferrer">Every time the Android app is opened, holochain takes ~10 seconds to boot up, so there is a long loading screen</a>.</li><li><a href="https://github.com/holochain/tx5/issues/87" target="_blank" rel="noreferrer">Go compiler issue on Android 11 or later</a>. p2p Shipyard solves this issue by providing a custom go toolchain, which is already included in the <code>devShells</code> and scaffolded projects described in throughout this documentation site, so <strong>if you use p2p Shipyard, this issue is not present at all</strong>.</li></ul><h3 id="ios" tabindex="-1">iOS <a class="header-anchor" href="#ios" aria-label="Permalink to &quot;iOS&quot;">​</a></h3><p>In development, holochain works as expected in iOS. But Apple prevents JIT compilation in iOS devices, so when a holochain app is published in the iOS store, it does not work. Thankfully there is already <a href="https://github.com/wasmerio/wasmer/issues/4486" target="_blank" rel="noreferrer">work in progress done by wasmer</a> to address this issue. Stay tuned for updates!</p><h2 id="well-okey-then-how-does-p2p-shipyard-help-me-now" tabindex="-1">Well, okey... Then how does p2p Shipyard help me now? <a class="header-anchor" href="#well-okey-then-how-does-p2p-shipyard-help-me-now" aria-label="Permalink to &quot;Well, okey... Then how does p2p Shipyard help me now?&quot;">​</a></h2><p>For now, you can build a desktop end-user hApp that your users can download and use, as all macOS, Linux and Windows are well supported. Furthermore, you can start experimenting with Android support, which has some UX downsides but is workable. After the issues with holochain mobile outlined above are resolved, you will be able to upgrade to a new version of the plugin to automatically get full mobile support in your hApp.</p><p>This is the way ourselves at <a href="https://darksoil.studio" target="_blank" rel="noreferrer">darksoil.studio</a> are building hApps right now. We are monitoring the issues at the core holochain infrastructure level, and in constant communication with the core holochain development team to help get them fixed. We hope that the remaining issues that prevent holochain to work on mobile outlined above get resolved soon, so that we can start deploying our holochain apps to end users.</p>',11),r=[i];function n(h,l,d,p,u,c){return t(),o("div",null,r)}const f=e(s,[["render",n]]);export{b as __pageData,f as default};
