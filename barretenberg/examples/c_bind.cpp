#include "c_bind.hpp"
#include "barretenberg/srs/global_crs.hpp"
#include "simple/simple.hpp"

using namespace proof_system::plonk::stdlib::types;

extern "C"
{
    const char* examples_simple_create_and_verify_proof(bool* valid)
    {
        try
        {
            auto ptrs = examples::simple::create_builder_and_composer(barretenberg::srs::get_crs_factory());
            auto proof = examples::simple::create_proof(ptrs);
            *valid = examples::simple::verify_proof(ptrs, proof);
            examples::simple::delete_builder_and_composer(ptrs);
            return nullptr;
        }
        catch (const std::exception &e)
        {
            return e.what(); // return the exception message
        }
    }
}
