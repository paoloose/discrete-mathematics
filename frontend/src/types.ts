import type { AstroInstance } from 'astro'

export type WeekMetadata = {
  title: string
  description: string
  img: string
}

export type WeekPage = {
  page: AstroInstance & {
    metadata: WeekMetadata
  }
}
