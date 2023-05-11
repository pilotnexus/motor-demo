import { PilotService } from './pilotService';
import { App } from 'vue';

const PilotServicePlugin = {
  install: (app: App, options: any) => {
    const pilotService = new PilotService(options);
    app.provide('pilotService', pilotService);
  },
};

export default PilotServicePlugin;

