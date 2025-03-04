# Forge

> Hammering code for a polished foundation.

## Generate Query

### Dependency

Depends on:
- `@tanstack/react-query`
- `@/shared/utils/fetch`


### Command

#### Default Query

```zsh
forge query [ENTITY]
```

Generates the following:

```ts
// @/shared/queries/useEntity.ts
import { useQuery } from '@tanstack/react-query'
import { api } from '@/shared/utils/fetch'
import { TEntity } from '@/shared/types/models/entity'

export type TResponse = TUser

export const QKEY_ENTITY = 'ENTITY'

export const ParamsSchema = z.object({})

export type TParams = z.infer<typeof ParamsSchema>
export type TRouteParams = {  }
export type TOpts = TParams & TRouteParams

export const useEntity = () => {
  return useQuery({
    queryKey: [QKEY_ENTITY],
    queryFn: async ({ signal }) => {
      const { data } = await api.get<TResponse>('/api', {
        signal
      })
      return data
    }
  })
}

```

#### Route Params Query

```zsh
forge query [ENTITY] -r id
```

Generates the following:

```ts
// @/shared/queries/useEntity.ts
import { useQuery } from '@tanstack/react-query'
import { api } from '@/shared/utils/fetch'
import { TEntity } from '@/shared/types/models/entity'

export type TResponse = TUser

export const QKEY_ENTITY = 'ENTITY'

export type TRouteParams = { ID: number | null }
export type TOpts = TRouteParams

export const useEntity = (opts: TOpts) => {
  return useQuery({
    queryKey: [QKEY_ENTITY, ID],
    queryFn: async ({ signal }) => {
      const { data } = await api.get<TResponse>('/api', {
        signal
      })
      return data
    },
    enabled: Boolean(ID)
  })
}

```

#### Params Query

```zsh
forge query [ENTITY] -p
```

Generates the following:

```ts
// @/shared/queries/useEntity.ts
import { useQuery } from '@tanstack/react-query'
import { api } from '@/shared/utils/fetch'
import { TEntity } from '@/shared/types/models/entity'

export type TResponse = TUser

export const QKEY_ENTITY = 'ENTITY'

export const ParamsSchema = z.object({})

export type TParams = z.infer<typeof ParamsSchema>
export type TOpts = TParams

export const useEntity = (opts: TOpts) => {
  return useQuery({
    queryKey: [QKEY_ENTITY, opts],
    queryFn: async ({ signal }) => {
      const { data } = await api.get<TResponse>('/api', {
        signal
      })
      return data
    },
    enabled: Boolean(opts)
  })
}

```

#### Params & Route Params Query

```zsh
forge query [ENTITY] -p -r id
```

Generates the following:

```ts
// @/shared/queries/useEntity.ts
import { useQuery } from '@tanstack/react-query'
import { api } from '@/shared/utils/fetch'
import { TEntity } from '@/shared/types/models/entity'

export type TResponse = TUser

export const QKEY_ENTITY = 'ENTITY'

export const ParamsSchema = z.object({})

export type TParams = z.infer<typeof ParamsSchema>
export type TRouteParams = { ID: number | null }
export type TOpts = TParams & TRouteParams

export const useEntity = (opts: TOpts) => {
  const { ID } = opts
  return useQuery({
    queryKey: [QKEY_ENTITY, opts],
    queryFn: async ({ signal }) => {
      const { data } = await api.get<TResponse>('/api', {
        signal
      })
      return data
    },
    enabled: Boolean(ID)
  })
}

```