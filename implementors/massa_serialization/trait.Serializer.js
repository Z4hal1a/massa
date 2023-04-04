(function() {var implementors = {
"massa_async_pool":[["impl Serializer&lt;<a class=\"struct\" href=\"massa_async_pool/message/struct.AsyncMessageTrigger.html\" title=\"struct massa_async_pool::message::AsyncMessageTrigger\">AsyncMessageTrigger</a>&gt; for <a class=\"struct\" href=\"massa_async_pool/message/struct.AsyncMessageTriggerSerializer.html\" title=\"struct massa_async_pool::message::AsyncMessageTriggerSerializer\">AsyncMessageTriggerSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_async_pool/message/struct.AsyncMessage.html\" title=\"struct massa_async_pool::message::AsyncMessage\">AsyncMessage</a>&gt; for <a class=\"struct\" href=\"massa_async_pool/message/struct.AsyncMessageSerializer.html\" title=\"struct massa_async_pool::message::AsyncMessageSerializer\">AsyncMessageSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_async_pool/changes/struct.AsyncPoolChanges.html\" title=\"struct massa_async_pool::changes::AsyncPoolChanges\">AsyncPoolChanges</a>&gt; for <a class=\"struct\" href=\"massa_async_pool/changes/struct.AsyncPoolChangesSerializer.html\" title=\"struct massa_async_pool::changes::AsyncPoolChangesSerializer\">AsyncPoolChangesSerializer</a>"],["impl Serializer&lt;(<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/cmp/struct.Reverse.html\" title=\"struct core::cmp::Reverse\">Reverse</a>&lt;<a class=\"struct\" href=\"https://docs.rs/num-rational/0.4/num_rational/struct.Ratio.html\" title=\"struct num_rational::Ratio\">Ratio</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>&gt;&gt;, Slot, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>)&gt; for <a class=\"struct\" href=\"massa_async_pool/message/struct.AsyncMessageIdSerializer.html\" title=\"struct massa_async_pool::message::AsyncMessageIdSerializer\">AsyncMessageIdSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html\" title=\"struct alloc::collections::btree::map::BTreeMap\">BTreeMap</a>&lt;(<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/cmp/struct.Reverse.html\" title=\"struct core::cmp::Reverse\">Reverse</a>&lt;<a class=\"struct\" href=\"https://docs.rs/num-rational/0.4/num_rational/struct.Ratio.html\" title=\"struct num_rational::Ratio\">Ratio</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>&gt;&gt;, Slot, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>), <a class=\"struct\" href=\"massa_async_pool/message/struct.AsyncMessage.html\" title=\"struct massa_async_pool::message::AsyncMessage\">AsyncMessage</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_async_pool/pool/struct.AsyncPoolSerializer.html\" title=\"struct massa_async_pool::pool::AsyncPoolSerializer\">AsyncPoolSerializer</a>"]],
"massa_bootstrap":[["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"enum\" href=\"massa_bootstrap/messages/enum.BootstrapClientMessage.html\" title=\"enum massa_bootstrap::messages::BootstrapClientMessage\">BootstrapClientMessage</a>&gt; for <a class=\"struct\" href=\"massa_bootstrap/messages/struct.BootstrapClientMessageSerializer.html\" title=\"struct massa_bootstrap::messages::BootstrapClientMessageSerializer\">BootstrapClientMessageSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"enum\" href=\"massa_bootstrap/messages/enum.BootstrapServerMessage.html\" title=\"enum massa_bootstrap::messages::BootstrapServerMessage\">BootstrapServerMessage</a>&gt; for <a class=\"struct\" href=\"massa_bootstrap/messages/struct.BootstrapServerMessageSerializer.html\" title=\"struct massa_bootstrap::messages::BootstrapServerMessageSerializer\">BootstrapServerMessageSerializer</a>"]],
"massa_consensus_exports":[["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_consensus_exports/bootstrapable_graph/struct.BootstrapableGraph.html\" title=\"struct massa_consensus_exports::bootstrapable_graph::BootstrapableGraph\">BootstrapableGraph</a>&gt; for <a class=\"struct\" href=\"massa_consensus_exports/bootstrapable_graph/struct.BootstrapableGraphSerializer.html\" title=\"struct massa_consensus_exports::bootstrapable_graph::BootstrapableGraphSerializer\">BootstrapableGraphSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_consensus_exports/export_active_block/struct.ExportActiveBlock.html\" title=\"struct massa_consensus_exports::export_active_block::ExportActiveBlock\">ExportActiveBlock</a>&gt; for <a class=\"struct\" href=\"massa_consensus_exports/export_active_block/struct.ExportActiveBlockSerializer.html\" title=\"struct massa_consensus_exports::export_active_block::ExportActiveBlockSerializer\">ExportActiveBlockSerializer</a>"]],
"massa_executed_ops":[["impl Serializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html\" title=\"struct alloc::collections::btree::map::BTreeMap\">BTreeMap</a>&lt;Slot, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html\" title=\"struct std::collections::hash::set::HashSet\">HashSet</a>&lt;OperationId, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;HashMapper&lt;OperationId&gt;&gt;&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_executed_ops/executed_ops/struct.ExecutedOpsSerializer.html\" title=\"struct massa_executed_ops::executed_ops::ExecutedOpsSerializer\">ExecutedOpsSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html\" title=\"struct std::collections::hash::map::HashMap\">HashMap</a>&lt;OperationId, (<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a>, Slot), <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;HashMapper&lt;OperationId&gt;&gt;&gt;&gt; for <a class=\"struct\" href=\"massa_executed_ops/ops_changes/struct.ExecutedOpsChangesSerializer.html\" title=\"struct massa_executed_ops::ops_changes::ExecutedOpsChangesSerializer\">ExecutedOpsChangesSerializer</a>"]],
"massa_final_state":[["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_final_state/state_changes/struct.StateChanges.html\" title=\"struct massa_final_state::state_changes::StateChanges\">StateChanges</a>&gt; for <a class=\"struct\" href=\"massa_final_state/state_changes/struct.StateChangesSerializer.html\" title=\"struct massa_final_state::state_changes::StateChangesSerializer\">StateChangesSerializer</a>"]],
"massa_hash":[["impl Serializer&lt;<a class=\"struct\" href=\"massa_hash/hash/struct.Hash.html\" title=\"struct massa_hash::hash::Hash\">Hash</a>&gt; for <a class=\"struct\" href=\"massa_hash/hash/struct.HashSerializer.html\" title=\"struct massa_hash::hash::HashSerializer\">HashSerializer</a>"]],
"massa_ledger_exports":[["impl Serializer&lt;<a class=\"struct\" href=\"massa_ledger_exports/ledger_entry/struct.LedgerEntry.html\" title=\"struct massa_ledger_exports::ledger_entry::LedgerEntry\">LedgerEntry</a>&gt; for <a class=\"struct\" href=\"massa_ledger_exports/ledger_entry/struct.LedgerEntrySerializer.html\" title=\"struct massa_ledger_exports::ledger_entry::LedgerEntrySerializer\">LedgerEntrySerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_ledger_exports/ledger_changes/struct.LedgerChanges.html\" title=\"struct massa_ledger_exports::ledger_changes::LedgerChanges\">LedgerChanges</a>&gt; for <a class=\"struct\" href=\"massa_ledger_exports/ledger_changes/struct.LedgerChangesSerializer.html\" title=\"struct massa_ledger_exports::ledger_changes::LedgerChangesSerializer\">LedgerChangesSerializer</a>"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, ST: Serializer&lt;T&gt;&gt; Serializer&lt;<a class=\"enum\" href=\"massa_ledger_exports/types/enum.SetOrKeep.html\" title=\"enum massa_ledger_exports::types::SetOrKeep\">SetOrKeep</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"massa_ledger_exports/types/struct.SetOrKeepSerializer.html\" title=\"struct massa_ledger_exports::types::SetOrKeepSerializer\">SetOrKeepSerializer</a>&lt;T, ST&gt;"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"massa_ledger_exports/types/trait.Applicable.html\" title=\"trait massa_ledger_exports::types::Applicable\">Applicable</a>&lt;V&gt;, V: <a class=\"trait\" href=\"massa_ledger_exports/types/trait.Applicable.html\" title=\"trait massa_ledger_exports::types::Applicable\">Applicable</a>&lt;V&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, ST: Serializer&lt;T&gt;, SV: Serializer&lt;V&gt;&gt; Serializer&lt;<a class=\"enum\" href=\"massa_ledger_exports/types/enum.SetUpdateOrDelete.html\" title=\"enum massa_ledger_exports::types::SetUpdateOrDelete\">SetUpdateOrDelete</a>&lt;T, V&gt;&gt; for <a class=\"struct\" href=\"massa_ledger_exports/types/struct.SetUpdateOrDeleteSerializer.html\" title=\"struct massa_ledger_exports::types::SetUpdateOrDeleteSerializer\">SetUpdateOrDeleteSerializer</a>&lt;T, V, ST, SV&gt;"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_ledger_exports/key/struct.Key.html\" title=\"struct massa_ledger_exports::key::Key\">Key</a>&gt; for <a class=\"struct\" href=\"massa_ledger_exports/key/struct.KeySerializer.html\" title=\"struct massa_ledger_exports::key::KeySerializer\">KeySerializer</a>"],["impl Serializer&lt;<a class=\"enum\" href=\"massa_ledger_exports/key/enum.KeyType.html\" title=\"enum massa_ledger_exports::key::KeyType\">KeyType</a>&gt; for <a class=\"struct\" href=\"massa_ledger_exports/key/struct.KeyTypeSerializer.html\" title=\"struct massa_ledger_exports::key::KeyTypeSerializer\">KeyTypeSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_ledger_exports/ledger_changes/struct.LedgerEntryUpdate.html\" title=\"struct massa_ledger_exports::ledger_changes::LedgerEntryUpdate\">LedgerEntryUpdate</a>&gt; for <a class=\"struct\" href=\"massa_ledger_exports/ledger_changes/struct.LedgerEntryUpdateSerializer.html\" title=\"struct massa_ledger_exports::ledger_changes::LedgerEntryUpdateSerializer\">LedgerEntryUpdateSerializer</a>"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, ST: Serializer&lt;T&gt;&gt; Serializer&lt;<a class=\"enum\" href=\"massa_ledger_exports/types/enum.SetOrDelete.html\" title=\"enum massa_ledger_exports::types::SetOrDelete\">SetOrDelete</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"massa_ledger_exports/types/struct.SetOrDeleteSerializer.html\" title=\"struct massa_ledger_exports::types::SetOrDeleteSerializer\">SetOrDeleteSerializer</a>&lt;T, ST&gt;"],["impl Serializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html\" title=\"struct alloc::collections::btree::map::BTreeMap\">BTreeMap</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;, <a class=\"enum\" href=\"massa_ledger_exports/types/enum.SetOrDelete.html\" title=\"enum massa_ledger_exports::types::SetOrDelete\">SetOrDelete</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_ledger_exports/ledger_changes/struct.DatastoreUpdateSerializer.html\" title=\"struct massa_ledger_exports::ledger_changes::DatastoreUpdateSerializer\">DatastoreUpdateSerializer</a>"]],
"massa_models":[["impl Serializer&lt;BitVec&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>, Lsb0&gt;&gt; for <a class=\"struct\" href=\"massa_models/serialization/struct.BitVecSerializer.html\" title=\"struct massa_models::serialization::BitVecSerializer\">BitVecSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_models/endorsement/struct.Endorsement.html\" title=\"struct massa_models::endorsement::Endorsement\">Endorsement</a>&gt; for <a class=\"struct\" href=\"massa_models/endorsement/struct.EndorsementSerializerLW.html\" title=\"struct massa_models::endorsement::EndorsementSerializerLW\">EndorsementSerializerLW</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_models/endorsement/struct.Endorsement.html\" title=\"struct massa_models::endorsement::Endorsement\">Endorsement</a>&gt; for <a class=\"struct\" href=\"massa_models/endorsement/struct.EndorsementSerializer.html\" title=\"struct massa_models::endorsement::EndorsementSerializer\">EndorsementSerializer</a>"],["impl Serializer&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/std/net/ip_addr/enum.IpAddr.html\" title=\"enum std::net::ip_addr::IpAddr\">IpAddr</a>&gt; for <a class=\"struct\" href=\"massa_models/serialization/struct.IpAddrSerializer.html\" title=\"struct massa_models::serialization::IpAddrSerializer\">IpAddrSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_models/ledger/struct.LedgerData.html\" title=\"struct massa_models::ledger::LedgerData\">LedgerData</a>&gt; for <a class=\"struct\" href=\"massa_models/ledger/struct.LedgerDataSerializer.html\" title=\"struct massa_models::ledger::LedgerDataSerializer\">LedgerDataSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_models/version/struct.Version.html\" title=\"struct massa_models::version::Version\">Version</a>&gt; for <a class=\"struct\" href=\"massa_models/version/struct.VersionSerializer.html\" title=\"struct massa_models::version::VersionSerializer\">VersionSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_models/ledger/struct.LedgerChanges.html\" title=\"struct massa_models::ledger::LedgerChanges\">LedgerChanges</a>&gt; for <a class=\"struct\" href=\"massa_models/ledger/struct.LedgerChangesSerializer.html\" title=\"struct massa_models::ledger::LedgerChangesSerializer\">LedgerChangesSerializer</a>"],["impl&lt;T, ST&gt; Serializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;T, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_models/serialization/struct.VecSerializer.html\" title=\"struct massa_models::serialization::VecSerializer\">VecSerializer</a>&lt;T, ST&gt;<span class=\"where fmt-newline\">where\n    ST: Serializer&lt;T&gt;,</span>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_models/bytecode/struct.Bytecode.html\" title=\"struct massa_models::bytecode::Bytecode\">Bytecode</a>&gt; for <a class=\"struct\" href=\"massa_models/bytecode/struct.BytecodeSerializer.html\" title=\"struct massa_models::bytecode::BytecodeSerializer\">BytecodeSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_models/serialization/struct.VecU8Serializer.html\" title=\"struct massa_models::serialization::VecU8Serializer\">VecU8Serializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_models/denunciation/struct.EndorsementDenunciation.html\" title=\"struct massa_models::denunciation::EndorsementDenunciation\">EndorsementDenunciation</a>&gt; for <a class=\"struct\" href=\"massa_models/denunciation/struct.EndorsementDenunciationSerializer.html\" title=\"struct massa_models::denunciation::EndorsementDenunciationSerializer\">EndorsementDenunciationSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_models/block/struct.Block.html\" title=\"struct massa_models::block::Block\">Block</a>&gt; for <a class=\"struct\" href=\"massa_models/block/struct.BlockSerializer.html\" title=\"struct massa_models::block::BlockSerializer\">BlockSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html\" title=\"struct alloc::collections::btree::map::BTreeMap\">BTreeMap</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_models/datastore/struct.DatastoreSerializer.html\" title=\"struct massa_models::datastore::DatastoreSerializer\">DatastoreSerializer</a>"],["impl&lt;T, ST&gt; Serializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html\" title=\"struct std::collections::hash::set::HashSet\">HashSet</a>&lt;T, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;<a class=\"struct\" href=\"massa_models/prehash/struct.HashMapper.html\" title=\"struct massa_models::prehash::HashMapper\">HashMapper</a>&lt;T&gt;&gt;&gt;&gt; for <a class=\"struct\" href=\"massa_models/serialization/struct.PreHashSetSerializer.html\" title=\"struct massa_models::serialization::PreHashSetSerializer\">PreHashSetSerializer</a>&lt;T, ST&gt;<span class=\"where fmt-newline\">where\n    ST: Serializer&lt;T&gt;,\n    T: <a class=\"trait\" href=\"massa_models/prehash/trait.PreHashed.html\" title=\"trait massa_models::prehash::PreHashed\">PreHashed</a>,</span>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_models/ledger/struct.LedgerChange.html\" title=\"struct massa_models::ledger::LedgerChange\">LedgerChange</a>&gt; for <a class=\"struct\" href=\"massa_models/ledger/struct.LedgerChangeSerializer.html\" title=\"struct massa_models::ledger::LedgerChangeSerializer\">LedgerChangeSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_models/denunciation/struct.BlockHeaderDenunciation.html\" title=\"struct massa_models::denunciation::BlockHeaderDenunciation\">BlockHeaderDenunciation</a>&gt; for <a class=\"struct\" href=\"massa_models/denunciation/struct.BlockHeaderDenunciationSerializer.html\" title=\"struct massa_models::denunciation::BlockHeaderDenunciationSerializer\">BlockHeaderDenunciationSerializer</a>"],["impl&lt;T, ST&gt; Serializer&lt;<a class=\"enum\" href=\"massa_models/streaming_step/enum.StreamingStep.html\" title=\"enum massa_models::streaming_step::StreamingStep\">StreamingStep</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"massa_models/streaming_step/struct.StreamingStepSerializer.html\" title=\"struct massa_models::streaming_step::StreamingStepSerializer\">StreamingStepSerializer</a>&lt;T, ST&gt;<span class=\"where fmt-newline\">where\n    ST: Serializer&lt;T&gt;,</span>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_models/slot/struct.Slot.html\" title=\"struct massa_models::slot::Slot\">Slot</a>&gt; for <a class=\"struct\" href=\"massa_models/slot/struct.SlotSerializer.html\" title=\"struct massa_models::slot::SlotSerializer\">SlotSerializer</a>"],["impl&lt;SL, L&gt; Serializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"struct\" href=\"massa_models/serialization/struct.StringSerializer.html\" title=\"struct massa_models::serialization::StringSerializer\">StringSerializer</a>&lt;SL, L&gt;<span class=\"where fmt-newline\">where\n    SL: Serializer&lt;L&gt;,\n    L: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt;,</span>"],["impl Serializer&lt;<a class=\"enum\" href=\"massa_models/denunciation/enum.Denunciation.html\" title=\"enum massa_models::denunciation::Denunciation\">Denunciation</a>&gt; for <a class=\"struct\" href=\"massa_models/denunciation/struct.DenunciationSerializer.html\" title=\"struct massa_models::denunciation::DenunciationSerializer\">DenunciationSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_models/rolls/struct.RollUpdate.html\" title=\"struct massa_models::rolls::RollUpdate\">RollUpdate</a>&gt; for <a class=\"struct\" href=\"massa_models/rolls/struct.RollUpdateSerializer.html\" title=\"struct massa_models::rolls::RollUpdateSerializer\">RollUpdateSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html\" title=\"struct std::collections::hash::set::HashSet\">HashSet</a>&lt;<a class=\"struct\" href=\"massa_models/operation/struct.OperationPrefixId.html\" title=\"struct massa_models::operation::OperationPrefixId\">OperationPrefixId</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;<a class=\"struct\" href=\"massa_models/prehash/struct.HashMapper.html\" title=\"struct massa_models::prehash::HashMapper\">HashMapper</a>&lt;<a class=\"struct\" href=\"massa_models/operation/struct.OperationPrefixId.html\" title=\"struct massa_models::operation::OperationPrefixId\">OperationPrefixId</a>&gt;&gt;&gt;&gt; for <a class=\"struct\" href=\"massa_models/operation/struct.OperationPrefixIdsSerializer.html\" title=\"struct massa_models::operation::OperationPrefixIdsSerializer\">OperationPrefixIdsSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"massa_models/secure_share/struct.SecureShare.html\" title=\"struct massa_models::secure_share::SecureShare\">SecureShare</a>&lt;<a class=\"struct\" href=\"massa_models/operation/struct.Operation.html\" title=\"struct massa_models::operation::Operation\">Operation</a>, <a class=\"struct\" href=\"massa_models/operation/struct.OperationId.html\" title=\"struct massa_models::operation::OperationId\">OperationId</a>&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_models/operation/struct.OperationsSerializer.html\" title=\"struct massa_models::operation::OperationsSerializer\">OperationsSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"massa_models/operation/struct.OperationId.html\" title=\"struct massa_models::operation::OperationId\">OperationId</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_models/operation/struct.OperationIdsSerializer.html\" title=\"struct massa_models::operation::OperationIdsSerializer\">OperationIdsSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_models/amount/struct.Amount.html\" title=\"struct massa_models::amount::Amount\">Amount</a>&gt; for <a class=\"struct\" href=\"massa_models/amount/struct.AmountSerializer.html\" title=\"struct massa_models::amount::AmountSerializer\">AmountSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_models/block_id/struct.BlockId.html\" title=\"struct massa_models::block_id::BlockId\">BlockId</a>&gt; for <a class=\"struct\" href=\"massa_models/block_id/struct.BlockIdSerializer.html\" title=\"struct massa_models::block_id::BlockIdSerializer\">BlockIdSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_models/clique/struct.Clique.html\" title=\"struct massa_models::clique::Clique\">Clique</a>&gt; for <a class=\"struct\" href=\"massa_models/clique/struct.CliqueSerializer.html\" title=\"struct massa_models::clique::CliqueSerializer\">CliqueSerializer</a>"],["impl&lt;T, ID&gt; Serializer&lt;<a class=\"struct\" href=\"massa_models/secure_share/struct.SecureShare.html\" title=\"struct massa_models::secure_share::SecureShare\">SecureShare</a>&lt;T, ID&gt;&gt; for <a class=\"struct\" href=\"massa_models/secure_share/struct.SecureShareSerializer.html\" title=\"struct massa_models::secure_share::SecureShareSerializer\">SecureShareSerializer</a><span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> + <a class=\"trait\" href=\"massa_models/secure_share/trait.SecureShareContent.html\" title=\"trait massa_models::secure_share::SecureShareContent\">SecureShareContent</a>,\n    ID: <a class=\"trait\" href=\"massa_models/secure_share/trait.Id.html\" title=\"trait massa_models::secure_share::Id\">Id</a>,</span>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_models/operation/struct.Operation.html\" title=\"struct massa_models::operation::Operation\">Operation</a>&gt; for <a class=\"struct\" href=\"massa_models/operation/struct.OperationSerializer.html\" title=\"struct massa_models::operation::OperationSerializer\">OperationSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_models/operation/struct.OperationId.html\" title=\"struct massa_models::operation::OperationId\">OperationId</a>&gt; for <a class=\"struct\" href=\"massa_models/operation/struct.OperationIdSerializer.html\" title=\"struct massa_models::operation::OperationIdSerializer\">OperationIdSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_models/block_header/struct.BlockHeader.html\" title=\"struct massa_models::block_header::BlockHeader\">BlockHeader</a>&gt; for <a class=\"struct\" href=\"massa_models/block_header/struct.BlockHeaderSerializer.html\" title=\"struct massa_models::block_header::BlockHeaderSerializer\">BlockHeaderSerializer</a>"],["impl Serializer&lt;<a class=\"enum\" href=\"massa_models/address/enum.Address.html\" title=\"enum massa_models::address::Address\">Address</a>&gt; for <a class=\"struct\" href=\"massa_models/address/struct.AddressSerializer.html\" title=\"struct massa_models::address::AddressSerializer\">AddressSerializer</a>"],["impl Serializer&lt;<a class=\"enum\" href=\"massa_models/operation/enum.OperationType.html\" title=\"enum massa_models::operation::OperationType\">OperationType</a>&gt; for <a class=\"struct\" href=\"massa_models/operation/struct.OperationTypeSerializer.html\" title=\"struct massa_models::operation::OperationTypeSerializer\">OperationTypeSerializer</a>"]],
"massa_module_cache":[["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"enum\" href=\"massa_module_cache/types/enum.ModuleMetadata.html\" title=\"enum massa_module_cache::types::ModuleMetadata\">ModuleMetadata</a>&gt; for <a class=\"struct\" href=\"massa_module_cache/types/struct.ModuleMetadataSerializer.html\" title=\"struct massa_module_cache::types::ModuleMetadataSerializer\">ModuleMetadataSerializer</a>"]],
"massa_network_exports":[["impl Serializer&lt;<a class=\"struct\" href=\"massa_network_exports/peers/struct.BootstrapPeers.html\" title=\"struct massa_network_exports::peers::BootstrapPeers\">BootstrapPeers</a>&gt; for <a class=\"struct\" href=\"massa_network_exports/peers/struct.BootstrapPeersSerializer.html\" title=\"struct massa_network_exports::peers::BootstrapPeersSerializer\">BootstrapPeersSerializer</a>"]],
"massa_network_worker":[["impl Serializer&lt;<a class=\"enum\" href=\"massa_network_worker/messages/enum.Message.html\" title=\"enum massa_network_worker::messages::Message\">Message</a>&gt; for <a class=\"struct\" href=\"massa_network_worker/messages/struct.MessageSerializer.html\" title=\"struct massa_network_worker::messages::MessageSerializer\">MessageSerializer</a>"]],
"massa_pos_exports":[["impl Serializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html\" title=\"struct std::collections::hash::map::HashMap\">HashMap</a>&lt;Address, Amount, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;HashMapper&lt;Address&gt;&gt;&gt;&gt; for <a class=\"struct\" href=\"massa_pos_exports/deferred_credits/struct.CreditsSerializer.html\" title=\"struct massa_pos_exports::deferred_credits::CreditsSerializer\">CreditsSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_pos_exports/pos_changes/struct.PoSChanges.html\" title=\"struct massa_pos_exports::pos_changes::PoSChanges\">PoSChanges</a>&gt; for <a class=\"struct\" href=\"massa_pos_exports/pos_changes/struct.PoSChangesSerializer.html\" title=\"struct massa_pos_exports::pos_changes::PoSChangesSerializer\">PoSChangesSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_pos_exports/cycle_info/struct.CycleInfo.html\" title=\"struct massa_pos_exports::cycle_info::CycleInfo\">CycleInfo</a>&gt; for <a class=\"struct\" href=\"massa_pos_exports/cycle_info/struct.CycleInfoSerializer.html\" title=\"struct massa_pos_exports::cycle_info::CycleInfoSerializer\">CycleInfoSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html\" title=\"struct std::collections::hash::map::HashMap\">HashMap</a>&lt;Address, <a class=\"struct\" href=\"massa_pos_exports/cycle_info/struct.ProductionStats.html\" title=\"struct massa_pos_exports::cycle_info::ProductionStats\">ProductionStats</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;HashMapper&lt;Address&gt;&gt;&gt;&gt; for <a class=\"struct\" href=\"massa_pos_exports/cycle_info/struct.ProductionStatsSerializer.html\" title=\"struct massa_pos_exports::cycle_info::ProductionStatsSerializer\">ProductionStatsSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_pos_exports/deferred_credits/struct.DeferredCredits.html\" title=\"struct massa_pos_exports::deferred_credits::DeferredCredits\">DeferredCredits</a>&gt; for <a class=\"struct\" href=\"massa_pos_exports/deferred_credits/struct.DeferredCreditsSerializer.html\" title=\"struct massa_pos_exports::deferred_credits::DeferredCreditsSerializer\">DeferredCreditsSerializer</a>"]],
"massa_serialization":[],
"massa_time":[["impl Serializer&lt;<a class=\"struct\" href=\"massa_time/struct.MassaTime.html\" title=\"struct massa_time::MassaTime\">MassaTime</a>&gt; for <a class=\"struct\" href=\"massa_time/struct.MassaTimeSerializer.html\" title=\"struct massa_time::MassaTimeSerializer\">MassaTimeSerializer</a>"]],
"massa_versioning_worker":[["impl Serializer&lt;<a class=\"struct\" href=\"massa_versioning_worker/versioning/struct.MipStoreRaw.html\" title=\"struct massa_versioning_worker::versioning::MipStoreRaw\">MipStoreRaw</a>&gt; for <a class=\"struct\" href=\"massa_versioning_worker/versioning_ser_der/struct.MipStoreRawSerializer.html\" title=\"struct massa_versioning_worker::versioning_ser_der::MipStoreRawSerializer\">MipStoreRawSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_versioning_worker/versioning/struct.MipStoreStats.html\" title=\"struct massa_versioning_worker::versioning::MipStoreStats\">MipStoreStats</a>&gt; for <a class=\"struct\" href=\"massa_versioning_worker/versioning_ser_der/struct.MipStoreStatsSerializer.html\" title=\"struct massa_versioning_worker::versioning_ser_der::MipStoreStatsSerializer\">MipStoreStatsSerializer</a>"],["impl Serializer&lt;<a class=\"enum\" href=\"massa_versioning_worker/versioning/enum.ComponentState.html\" title=\"enum massa_versioning_worker::versioning::ComponentState\">ComponentState</a>&gt; for <a class=\"struct\" href=\"massa_versioning_worker/versioning_ser_der/struct.ComponentStateSerializer.html\" title=\"struct massa_versioning_worker::versioning_ser_der::ComponentStateSerializer\">ComponentStateSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_versioning_worker/versioning/struct.Advance.html\" title=\"struct massa_versioning_worker::versioning::Advance\">Advance</a>&gt; for <a class=\"struct\" href=\"massa_versioning_worker/versioning_ser_der/struct.AdvanceSerializer.html\" title=\"struct massa_versioning_worker::versioning_ser_der::AdvanceSerializer\">AdvanceSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_versioning_worker/versioning/struct.MipState.html\" title=\"struct massa_versioning_worker::versioning::MipState\">MipState</a>&gt; for <a class=\"struct\" href=\"massa_versioning_worker/versioning_ser_der/struct.MipStateSerializer.html\" title=\"struct massa_versioning_worker::versioning_ser_der::MipStateSerializer\">MipStateSerializer</a>"],["impl Serializer&lt;<a class=\"struct\" href=\"massa_versioning_worker/versioning/struct.MipInfo.html\" title=\"struct massa_versioning_worker::versioning::MipInfo\">MipInfo</a>&gt; for <a class=\"struct\" href=\"massa_versioning_worker/versioning_ser_der/struct.MipInfoSerializer.html\" title=\"struct massa_versioning_worker::versioning_ser_der::MipInfoSerializer\">MipInfoSerializer</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()