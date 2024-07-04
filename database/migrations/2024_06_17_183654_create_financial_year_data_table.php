<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        Schema::create('financial_year_data', function (Blueprint $table) {
            $table->id();
            $table->string('year');
            $table->string('industry_aggregation_nz');
            $table->string('industry_code_nz');
            $table->string('industry_name_nz');
            $table->string('units');
            $table->string('variable_code');
            $table->string('variable_name');
            $table->string('variable_category');
            $table->string('value');
            $table->string('industry_code_anz');
            $table->timestamp('created_at')->useCurrent();
            $table->timestamp('updated_at')->useCurrent()->useCurrentOnUpdate();
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('financial_year_data');
    }
};
