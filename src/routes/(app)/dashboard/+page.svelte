<script lang="ts">
  import { useSession } from "#lib/auth-client";

  const session = useSession();
</script>

<p>
  {$session.data?.user.name}
</p>