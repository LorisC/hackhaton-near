<template>
    <el-container id="app">
        <el-header >
            <div class="header">
                <h1>No Trash Alliance</h1>
                <div class="login-logout-button">
                    <el-button v-if="!isSignIn" @click="login">Login</el-button>
                    <div v-else class="logout-button">
                        <h2>{{$wallet.getAccountId()}}</h2>
                        <el-button class="button" v-if="isSignIn" @click="logout">Logout</el-button>
                    </div>
                </div>
            </div>
            <el-menu mode="horizontal" :style="{width: '100%'}" :router="true">
                <el-menu-item index="account">Account</el-menu-item>
                <el-menu-item index="market">Market Place</el-menu-item>
            </el-menu>
        </el-header>
        <el-main class="main">
            <router-view></router-view>
        </el-main>
    </el-container>
</template>

<script>
    export default {
        name: 'App',
        data() {
            return {
                userinfo: {},
                isSignIn: false
            }
        },
        methods: {
            login() {
                this.$wallet.requestSignIn(this.$contractName, "Track trash",);
                /*  this.$trackerFactoryContract.account = new this.$nearAPI.Account(this.$near.connection, this.$wallet.getAccountId());
                  this.$trackerFactoryContract.sender = this.$wallet.getAccountId()*/
            },

            viewInfo(company) {
                this.selectedCompany = company;
                this.displayInfo = true;
            },
            async loadCompanies() {
                this.loadingCompanies = true;
                this.companies.length = 0;
                const companies = await this.$trackerFactoryContract.get_green_companies();
                this.companies.push(...companies);
                this.loadingCompanies = false;
            },
            logout() {
                this.$wallet.signOut();
                window.wallet = this.$wallet;
                this.$router.push("/");
            },

        },
        async mounted() {
            await this.$nextTick();
            if (this.$wallet.isSignedIn()) {
                this.$trackerFactoryContract.account = new this.$nearAPI.Account(this.$near.connection, this.$wallet.getAccountId());
                this.$trackerFactoryContract.sender = this.$wallet.getAccountId();
                this.isSignIn = true;

            }
        }
    }
</script>

<style scoped>
    #app {
        font-size: 18px;
    }

    .header {
        color: lightslategrey;
        display: grid;
        grid-template-columns: 25% 75%;

        align-content: center;
        justify-content: start;
    }

    .login-logout-button {
        justify-self: end;
        align-self: center;
    }

    .logout-button {
        display: grid;
        grid-template-columns: auto auto;
        column-gap: .2em;
    }

    .button {
        height: 2.7em;
        align-self: center;
        margin: .2em;
    }

    .main{
        margin-top: 3em;

    }
</style>
