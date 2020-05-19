import Vue from 'vue'
import VueRouter from 'vue-router'
import Home from '../views/Home.vue'
import Account from "../views/Account";
import MarketPlace from "../views/MarketPlace";

Vue.use(VueRouter)

  const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
    {
      path: "/account",
      name: "Account",
      component: Account
    },
    {
      path:"/market",
      name: "market",
      component: MarketPlace

    },
    {
      path: '*',
      name: "404 Not Found",
      component: () => import(/* webpackChunkName: "about" */ '../views/NotFound.vue')
    }
];

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes
});

export default router
