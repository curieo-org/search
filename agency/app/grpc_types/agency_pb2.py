# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: agency.proto
# Protobuf Python Version: 4.25.1
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder

# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(
    b'\n\x0c\x61gency.proto\x12\x06\x61gency"\x1e\n\rSearchRequest\x12\r\n\x05query\x18\x01 \x01(\t"v\n\x06Source\x12\x0b\n\x03url\x18\x01 \x01(\t\x12.\n\x08metadata\x18\x02 \x03(\x0b\x32\x1c.agency.Source.MetadataEntry\x1a/\n\rMetadataEntry\x12\x0b\n\x03key\x18\x01 \x01(\t\x12\r\n\x05value\x18\x02 \x01(\t:\x02\x38\x01"Q\n\x0eSearchResponse\x12\x0e\n\x06status\x18\x01 \x01(\x05\x12\x0e\n\x06result\x18\x02 \x01(\t\x12\x1f\n\x07sources\x18\x03 \x03(\x0b\x32\x0e.agency.Source2[\n\rAgencyService\x12J\n\x19pubmed_bioxriv_web_search\x12\x15.agency.SearchRequest\x1a\x16.agency.SearchResponseb\x06proto3'
)

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, "agency_pb2", _globals)
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    _globals["_SOURCE_METADATAENTRY"]._options = None
    _globals["_SOURCE_METADATAENTRY"]._serialized_options = b"8\001"
    _globals["_SEARCHREQUEST"]._serialized_start = 24
    _globals["_SEARCHREQUEST"]._serialized_end = 54
    _globals["_SOURCE"]._serialized_start = 56
    _globals["_SOURCE"]._serialized_end = 174
    _globals["_SOURCE_METADATAENTRY"]._serialized_start = 127
    _globals["_SOURCE_METADATAENTRY"]._serialized_end = 174
    _globals["_SEARCHRESPONSE"]._serialized_start = 176
    _globals["_SEARCHRESPONSE"]._serialized_end = 257
    _globals["_AGENCYSERVICE"]._serialized_start = 259
    _globals["_AGENCYSERVICE"]._serialized_end = 350
# @@protoc_insertion_point(module_scope)
