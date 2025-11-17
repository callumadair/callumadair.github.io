<script setup lang="ts">
import type { NavigationMenuItem } from '@nuxt/ui';

const collapsed = ref<boolean>(false);
const collapseButtonIcon = ref("lucide:panel-left-close");
const toggleCollapse = () => {
  console.log("expanding");
  if (collapsed && collapsed.value === true) {
    collapsed.value = false;
    collapseButtonIcon.value = "lucide:panel-left-close";
  } else {
    collapsed.value = true;
    collapseButtonIcon.value = "lucide:panel-left-open";
  }
};

const items = ref<NavigationMenuItem[][]>([
  [
    {
      label: "Home",
      icon: "lucide:house",
      to: "/home/"
    },
    {
      label: "Personal Projects",
      icon: "lucide:code",
      to: "/projects/"
    },
    {
      label: "Software I Like",
      icon: "lucide:folder-code",
      to: "/software/",
    },
    {
      label: "Reading List",
      icon: "lucide:book",
      to: "/books/"
    }
  ],
  [
    {
      label: "Landing Page",
      icon: "lucide:star",
      to: "/"
    },
    {
      label: "Github",
      icon: "lucide:github",
      to: "https://github.com/callumadair/callumadair.github.io",
      target: "_blank"
    },
    {
      label: "Gitlab",
      icon: "lucide:gitlab",
      to: "https://gitlab.com/callumadair/callumadair-github-io/-/tree/convert-to-vue-js?ref_type=heads",
      target: "_blank"
    },
    {
      label: "Theme",
      icon: "lucide:palette",
      popover: {
        mode: "hover"
      },
      children: [
        {
          label: "Dark",
        }
        , {
          label: "Light",
        }
      ]
    }
  ]
]);

const iconSize = 20;
const textSize = "text-sm";
</script>

<template>
  <aside class="p-2 h-svh flex flex-col center border-r border-grey-accent-500 items-start">
    <div class="flex flex-row w-full">
      <button @click="toggleCollapse"
        class="p-3 rounded-xs flex grow justify-start items-center bg-ui hover:text-emerald-500 hover:bg-muted hover:cursor-pointer space-x-2.5">
        <Icon :name="collapseButtonIcon" :size="iconSize" />
        <Transition>
          <span v-if="!collapsed" :class="textSize">
            Collapse
          </span>
        </Transition>
      </button>
    </div>
    <div class="flex flex-col grow justify-between w-full">
      <div v-for="section in items">
        <NuxtLink v-for="item in section" :class="item.class" active-class="text-blue-400"
          class="p-3 rounded-xs hover:bg-muted flex grow items-center space-x-2.5" @click="item.onSelect" :to="item.to"
          :target="item.target">
          <Icon v-if="item.icon && (typeof item.icon) === 'string'" :name="item.icon" :size="iconSize" />
          <Transition>
            <span v-if="!collapsed && item.label && (typeof item.label) === 'string'" class="text-nowrap"
              :class="textSize">
              {{ item.label }}
            </span>
          </Transition>
        </NuxtLink>
      </div>
    </div>

  </aside>
</template>


<style>
.v-enter-active {
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  transform-origin: left center;
  overflow: hidden;
}

.v-leave-active {
  transition: all 0.15s cubic-bezier(0.4, 0, 0.6, 1);
  transform-origin: left center;
  overflow: hidden;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
  max-width: 0;
  transform: translateX(-10px);
}

.v-enter-to,
.v-leave-from {
  opacity: 1;
  max-width: 200px;
}
</style>