import Vue from 'vue'
import App from './App.vue'
import * as nearAPI from "near-api-js";

Vue.config.productionTip = false

const CONTRACT_NAME = "tracker_factory.testnet";

async function initNear() {
    const nearConfig = {
        networkId: 'default',
        nodeUrl: 'https://rpc.nearprotocol.com',
        contractName: CONTRACT_NAME,
        walletUrl: 'https://wallet.nearprotocol.com',
    };
    const keyStore = new nearAPI.keyStores.BrowserLocalStorageKeyStore();
    Vue.prototype.$nearAPI = nearAPI;
    Vue.prototype.$near = await nearAPI.connect(Object.assign({keyStore, deps: {keyStore}}, nearConfig));
    Vue.prototype.$wallet = new nearAPI.WalletConnection(Vue.prototype.$near);
    const account = new nearAPI.Account(Vue.prototype.$near.connection, CONTRACT_NAME);
    Vue.prototype.$trackerFactoryContract = new nearAPI.Contract(account, CONTRACT_NAME, {
        viewMethods: ['get_tracker_created'],
        changeMethods: ['create_tracker'],
        sender: Vue.prototype.$wallet.getAccountId()
    });
}

initNear().then(() => {

    new Vue({
        render: h => h(App),
    }).$mount('#app');

})
