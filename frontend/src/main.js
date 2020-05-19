import Vue from 'vue'
import App from './App.vue'
import * as nearAPI from "near-api-js";
import './plugins/element.js'
import {CONTRACT_NAME} from "./constants/Constants";
import router from './router'
import store from './store'

Vue.config.productionTip = false


async function initNear() {
    const nearConfig = {
        networkId: 'default',
        nodeUrl: 'https://rpc.nearprotocol.com',
        contractName: CONTRACT_NAME,
        walletUrl: 'https://wallet.nearprotocol.com',
    };
    const keyStore = new nearAPI.keyStores.BrowserLocalStorageKeyStore();
    Vue.prototype.$contractName = CONTRACT_NAME;
    Vue.prototype.$nearAPI = nearAPI;
    Vue.prototype.$near = await nearAPI.connect(Object.assign({keyStore, deps: {keyStore}}, nearConfig));
    Vue.prototype.$wallet = new nearAPI.WalletConnection(Vue.prototype.$near);
    const account = new nearAPI.Account(Vue.prototype.$near.connection, CONTRACT_NAME);
    Vue.prototype.$trackerFactoryContract = new nearAPI.Contract(account, CONTRACT_NAME, {
        viewMethods: ['get_tracker_created', "get_nb_green_companies", "get_nb_accounts"],
        changeMethods: [
            'create_tracker', 'register', "get_green_companies",
            "get_company_info", "get_account_tracker", "is_registered",
            "get_account_info"
        ],
        sender: Vue.prototype.$wallet.getAccountId()
    });
}

initNear().then(() => {

    new Vue({
        router,
        store,
        render: h => h(App)
    }).$mount('#app');

});
