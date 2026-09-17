import base from './playwright.config.js';
export default {...base,testMatch:['expanded.spec.js'],use:{...base.use,baseURL:'http://127.0.0.1:8177'},webServer:{command:'python3 -m http.server 8177 --bind 127.0.0.1',url:'http://127.0.0.1:8177/',reuseExistingServer:false}};
