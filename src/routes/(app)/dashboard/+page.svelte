<script lang="ts">
  import { useSession } from "#lib/auth-client";

const session = useSession();
</script>

<main>
  {$session.data?.user.name}
</main>