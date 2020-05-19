<template>
  <div class="home">
    <div>
      <p>There is currently {{trashTracker.length}} trash tracked around the world</p>
      <p>There is currently {{nb_companies}} companies in the tracking network</p>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Home',
  data(){
    return {
      trashTracker: [],
      nb_companies: 0,
    }
  },
  methods: {
    async loadFactoryInfo() {
      this.trashTracker.length = 0;
      this.trashTracker.push(...(await this.$trackerFactoryContract.get_tracker_created()))
      this.nb_companies = await this.$trackerFactoryContract.get_nb_accounts()
    }
  },
  async mounted() {
    await this.$nextTick();
    await this.loadFactoryInfo();
  }
}
</script>
<style scoped>
  .home{
    margin-top: 60px;
  }
</style>
