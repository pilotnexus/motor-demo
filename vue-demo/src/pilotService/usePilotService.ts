// usePilotService.ts

import { pilotService } from './pilotServicePlugin';

export function usePilotService(serviceName?: string) {
  const get = (variable: string) => {
    return pilotService.getValue(variable, serviceName);
  };

  const set = (variable: string, value: any) => {
    return pilotService.setValue(variable, value, serviceName);
  };

  return { get, set };
}

