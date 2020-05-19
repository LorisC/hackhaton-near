import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

export default new Vuex.Store({
    state: {
        trackers: {},
        tracker_buyOwner: {},
        registered: false
    },
    mutations: {
        set_registered(state, value){
          state.registered = value;
        },
        set_tracker(state, {id, tracker}) {
            if (state.tracker_buyOwner[tracker.owner] === undefined)
                state.tracker_buyOwner[tracker.owner] = {};
            state.trackers[id] = tracker;
            state.tracker_buyOwner[tracker.owner][id] = tracker;
        },
        set_trackers(state, trackers) {
            for (let key in trackers) {
                if (trackers[key] !== undefined) {
                    let tracker = trackers[key];
                    if (state.tracker_buyOwner[tracker.owner] === undefined)
                        state.tracker_buyOwner[tracker.owner] = {};
                    state.trackers[key] = tracker;
                    state.tracker_buyOwner[tracker.owner][key] = tracker;
                }
            }

        }
    },
    modules: {}
})
