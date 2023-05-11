<!-- MachineState.vue -->
<template>
    <v-container>
        <v-row align="center" justify="center">
            <v-col cols="12" class="text-center">
                <h1>State: {{ state_name }} ({{state}})</h1>
            </v-col>
        </v-row>

        <v-row justify="center">
            <v-col cols="12" md="6">
                <h3>Lateral Axis</h3>
                <v-progress-linear
                    v-model="lat_position"
                    color="blue-grey"
                    height="25"
                    max="200"
                >
                    <template v-slot:default="{ value }">
                        <strong>{{ lat_position}} steps</strong>
                    </template>
                </v-progress-linear>
            </v-col>
        </v-row>
        
        <v-row justify="center">
            <v-col cols="12" md="6">
                <h3>Longitudinal Axis</h3>
                <v-progress-linear
                    v-model="long_position"
                    color="blue-grey"
                    height="25"
                    max="200"
                >
                    <template v-slot:default="{ value }">
                        <strong>{{ long_position}} steps</strong>
                    </template>
                </v-progress-linear>
            </v-col>
        </v-row>

        <v-row justify="center">
            <v-col cols="12" md="4">
                <v-btn 
                    color="primary" 
                    :disabled="state!==0" 
                    v-set="['start_machine', true]"
                    block
                >
                    Start Machine
                </v-btn>
            </v-col>
        </v-row>

        <v-row justify="center">
            <v-col cols="12" md="4">
                <v-btn 
                    color="primary" 
                    :disabled="state!==1" 
                    v-set="['start_demo', true]"
                    block
                >
                    Start Demo
                </v-btn>
            </v-col>
        </v-row>
    </v-container>
</template>
  
  <script lang="ts">
  import { defineComponent, reactive, computed, toRefs, isReactive, watch } from 'vue';
  import { useSubscribe } from '../pilotService/useSubscribe';

export default defineComponent({
  setup() {
    const state = useSubscribe<int>('state', -1, value => parseInt(value));
    const lat_position = useSubscribe<int>('lateral_axis.position', 0, value => parseInt(value));
    const long_position = useSubscribe<int>('longitudinal_axis.position', 0, value => parseInt(value));

    const state_name = computed(() => {
      switch(parseInt(state.value)) {
        case 0:
          return "Initial State";
          break;
        case 1:
          return "Idle State";
          break;
        case 2:
          return "Moving State";
          break;
        case 3:
          return "Error State";
          break;
        default:
          return "Unknown State";
          }
      });

    return {
        state,
        state_name,
        lat_position,
        long_position
    };
  },
});
  </script>
