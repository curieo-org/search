# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: rag.proto
# Protobuf Python Version: 4.25.1
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\trag.proto\x12\x03rag\"J\n\rSearchRequest\x12\r\n\x05query\x18\x01 \x01(\t\x12*\n\x0eroute_category\x18\x02 \x01(\x0e\x32\x12.rag.RouteCategory\"&\n\x08Metadata\x12\x0b\n\x03key\x18\x01 \x01(\t\x12\r\n\x05value\x18\x02 \x01(\t\"6\n\x06Source\x12\x0b\n\x03url\x18\x01 \x01(\t\x12\x1f\n\x08metadata\x18\x02 \x03(\x0b\x32\r.rag.Metadata\"N\n\x0eSearchResponse\x12\x0e\n\x06status\x18\x01 \x01(\x05\x12\x0e\n\x06result\x18\x02 \x01(\t\x12\x1c\n\x07sources\x18\x03 \x03(\x0b\x32\x0b.rag.Source*W\n\rRouteCategory\x12\x12\n\x0e\x43linicalTrials\x10\x00\x12\x08\n\x04\x44RUG\x10\x01\x12\x16\n\x12PUBMED_BIOXRIV_WEB\x10\x02\x12\x10\n\x0cNOT_SELECTED\x10\x03\x32?\n\nRagService\x12\x31\n\x06search\x12\x12.rag.SearchRequest\x1a\x13.rag.SearchResponseb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'rag_pb2', _globals)
if _descriptor._USE_C_DESCRIPTORS == False:
  DESCRIPTOR._options = None
  _globals['_ROUTECATEGORY']._serialized_start=270
  _globals['_ROUTECATEGORY']._serialized_end=357
  _globals['_SEARCHREQUEST']._serialized_start=18
  _globals['_SEARCHREQUEST']._serialized_end=92
  _globals['_METADATA']._serialized_start=94
  _globals['_METADATA']._serialized_end=132
  _globals['_SOURCE']._serialized_start=134
  _globals['_SOURCE']._serialized_end=188
  _globals['_SEARCHRESPONSE']._serialized_start=190
  _globals['_SEARCHRESPONSE']._serialized_end=268
  _globals['_RAGSERVICE']._serialized_start=359
  _globals['_RAGSERVICE']._serialized_end=422
# @@protoc_insertion_point(module_scope)
