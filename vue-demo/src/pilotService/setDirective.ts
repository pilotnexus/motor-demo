// setDirective.ts
// setDirective.ts
import type { Directive } from 'vue';
import type { PilotServicePlugin } from './PilotServicePlugin';

export function createSetDirective(pilotServicePlugin: PilotServicePlugin): Directive {
    return {
        mounted(el, binding) {
            el.addEventListener('click', () => {
                const [path, value] = binding.value;
                pilotServicePlugin.setValue(path, value);
            });
        },
    };
}
