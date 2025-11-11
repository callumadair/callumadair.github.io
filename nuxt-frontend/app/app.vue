<script setup lang="ts">
import type { NavigationMenuItem } from '@nuxt/ui';

</script>

<template>
  <UApp>
    <div class="flex flex-row">
      <PortfolioMenu />

      <NuxtPage class="flex grow p-2" />

      <UFooter class="absolute bottom-0 text-center flex w-full">
        <span class="text-xs">
          Copyright Callum Adair 2025
          <sup>
            <Icon name="lucide:copyright" />
          </sup>
        </span>
      </UFooter>
    </div>
  </UApp>
</template>
