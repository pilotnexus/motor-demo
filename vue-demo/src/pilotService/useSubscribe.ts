import { inject, onUnmounted, type Ref, type UnwrapRef, ref, nextTick } from 'vue';
import type { PilotServicePlugin } from './pilotServicePlugin';

export function useSubscribe<T>(path: string, defaultValue: T, convert?: (value: any) => T): Ref<UnwrapRef<T>> {
    const pilotServicePlugin = inject<PilotServicePlugin>('pilotService');

    const target = ref(defaultValue);

    if (pilotServicePlugin) {
        const unsubscribe = pilotServicePlugin.subscribe(path, (value: any) => {
            // console.log(`updating ${path} to ${value}`);
            target.value = convert ? convert(value) : value;
        });

        onUnmounted(() => {
            unsubscribe();
        });
    } else {
        console.error("pilotServicePlugin missing!");
    }

    return target;
}
